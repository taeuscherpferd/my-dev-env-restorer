use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, anyhow};

use crate::cli::Cli;
use crate::manifest::{
    ConfigEntry, Platform, ProgramEntry, load_config_manifest, load_program_manifest,
};
use crate::repo::{discover_repo_root, expand_target_path};

pub fn run(cli: Cli) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to read the current directory")?;
    let repo_root = discover_repo_root(&current_dir)?;
    let platform = Platform::current();

    if cli.configs {
        sync_configs(&repo_root, platform, SyncDirection::Apply, cli.dry_run)?;
    }

    if cli.pull {
        sync_configs(&repo_root, platform, SyncDirection::Pull, cli.dry_run)?;
    }

    if cli.links {
        open_program_links(&repo_root, platform, cli.dry_run)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum SyncDirection {
    Apply,
    Pull,
}

fn sync_configs(
    repo_root: &Path,
    platform: Platform,
    direction: SyncDirection,
    dry_run: bool,
) -> Result<()> {
    let manifest_path = repo_root.join("manifests").join("configs.toml");
    let manifest = load_config_manifest(&manifest_path)?;

    let mut changed = 0usize;
    let mut skipped = 0usize;

    for entry in &manifest.config {
        let Some(target_raw) = entry.target_for_platform(platform) else {
            skipped += 1;
            continue;
        };

        let machine_path = expand_target_path(target_raw)?;

        match direction {
            SyncDirection::Apply => {
                let generated = build_repo_output(repo_root, entry, platform)?;
                write_generated_file(&generated, &machine_path, entry, dry_run)?;
                changed += 1;
            }
            SyncDirection::Pull => {
                if entry.is_generated_for_platform(platform) {
                    println!(
                        "skip {}: generated from shared base plus fragment overlays; pull is not automatic",
                        entry.id
                    );
                    skipped += 1;
                    continue;
                }

                let Some(source_relative) = entry.base_source_for_platform(platform) else {
                    return Err(anyhow!(
                        "Config '{}' has no source for platform {}",
                        entry.id,
                        platform.as_key()
                    ));
                };

                let repo_path = repo_root.join(source_relative);

                if !machine_path.exists() {
                    println!(
                        "skip {}: target does not exist at {}",
                        entry.id,
                        machine_path.display()
                    );
                    skipped += 1;
                    continue;
                }

                copy_file(&machine_path, &repo_path, entry, "machine", "repo", dry_run)?;
                changed += 1;
            }
        }
    }

    let action = match direction {
        SyncDirection::Apply => "applied",
        SyncDirection::Pull => "pulled",
    };

    println!("Config sync complete: {changed} {action}, {skipped} skipped.");
    Ok(())
}

fn build_repo_output(repo_root: &Path, entry: &ConfigEntry, platform: Platform) -> Result<String> {
    let Some(base_relative) = entry.base_source_for_platform(platform) else {
        return Err(anyhow!(
            "Config '{}' has no source for platform {}",
            entry.id,
            platform.as_key()
        ));
    };

    let mut output = read_repo_file(&repo_root.join(base_relative), entry, "base source")?;

    for fragment_relative in entry.fragments_for_platform(platform) {
        let fragment = read_repo_file(&repo_root.join(fragment_relative), entry, "fragment")?;
        append_fragment(&mut output, &fragment);
    }

    Ok(output)
}

fn read_repo_file(path: &Path, entry: &ConfigEntry, label: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read config '{}' {} at {}",
            entry.id,
            label,
            path.display()
        )
    })
}

fn append_fragment(output: &mut String, fragment: &str) {
    if !output.ends_with('\n') {
        output.push('\n');
    }

    output.push('\n');
    output.push_str(fragment);

    if !output.ends_with('\n') {
        output.push('\n');
    }
}

fn write_generated_file(
    content: &str,
    destination: &Path,
    entry: &ConfigEntry,
    dry_run: bool,
) -> Result<()> {
    let description = entry
        .description
        .as_deref()
        .unwrap_or("no description provided");

    if dry_run {
        println!(
            "dry-run {}: generated output -> {} ({description})",
            entry.id,
            destination.display()
        );
        return Ok(());
    }

    let parent = destination.parent().ok_or_else(|| {
        anyhow!(
            "Config '{}' resolved to a target without a parent directory: {}",
            entry.id,
            destination.display()
        )
    })?;

    fs::create_dir_all(parent).with_context(|| {
        format!(
            "Failed to create the parent directory for {}",
            destination.display()
        )
    })?;

    fs::write(destination, content).with_context(|| {
        format!(
            "Failed to write generated config '{}' to {}",
            entry.id,
            destination.display()
        )
    })?;

    println!("generated {} -> {}", entry.id, destination.display());
    println!("synced to machine: {description}");
    Ok(())
}

fn copy_file(
    source: &Path,
    destination: &Path,
    entry: &ConfigEntry,
    source_label: &str,
    destination_label: &str,
    dry_run: bool,
) -> Result<()> {
    if !source.exists() {
        return Err(anyhow!(
            "Config '{}' expected a source file at {}",
            entry.id,
            source.display()
        ));
    }

    let description = entry
        .description
        .as_deref()
        .unwrap_or("no description provided");

    if dry_run {
        println!(
            "dry-run {}: {} -> {} ({description})",
            entry.id,
            source.display(),
            destination.display()
        );
        return Ok(());
    }

    let parent = destination.parent().ok_or_else(|| {
        anyhow!(
            "Config '{}' resolved to a target without a parent directory: {}",
            entry.id,
            destination.display()
        )
    })?;

    fs::create_dir_all(parent).with_context(|| {
        format!(
            "Failed to create the parent directory for {}",
            destination.display()
        )
    })?;

    fs::copy(source, destination).with_context(|| {
        format!(
            "Failed to copy config '{}' from {} to {}",
            entry.id,
            source.display(),
            destination.display()
        )
    })?;

    println!(
        "{} {}: {} -> {}",
        source_label,
        entry.id,
        source.display(),
        destination.display()
    );
    println!("synced to {destination_label}: {description}");
    Ok(())
}

fn open_program_links(repo_root: &Path, platform: Platform, dry_run: bool) -> Result<()> {
    let manifest_path = repo_root.join("manifests").join("programs.toml");
    let manifest = load_program_manifest(&manifest_path)?;

    let matching_programs: Vec<&ProgramEntry> = manifest
        .program
        .iter()
        .filter(|program| program.matches_platform(platform))
        .collect();

    for program in &matching_programs {
        if dry_run {
            println!("dry-run open link: {} -> {}", program.name, program.url);
            continue;
        }

        open_in_browser(&program.url)
            .with_context(|| format!("Failed to open the link for {}", program.name))?;

        if let Some(notes) = &program.notes {
            println!("opened {} ({notes})", program.name);
        } else {
            println!("opened {}", program.name);
        }
    }

    println!("Opened {} program link(s).", matching_programs.len());
    Ok(())
}

fn open_in_browser(url: &str) -> Result<()> {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", "", url]).status()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else {
        Command::new("xdg-open").arg(url).status()
    }
    .with_context(|| format!("Failed to spawn a browser opener for {url}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Browser opener exited unsuccessfully for {url}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::{ConfigEntry, Platform, ProgramEntry};
    use std::collections::BTreeMap;

    #[test]
    fn prefers_platform_overlay_when_present() {
        let entry = ConfigEntry {
            id: "vimrc".to_owned(),
            description: None,
            shared: Some("configs/shared/vim/.vimrc".to_owned()),
            overlays: BTreeMap::from([(
                "windows".to_owned(),
                "configs/windows/vim/.vimrc".to_owned(),
            )]),
            fragments: Vec::new(),
            platform_fragments: BTreeMap::new(),
            targets: BTreeMap::new(),
        };

        assert_eq!(
            entry.base_source_for_platform(Platform::Windows),
            Some("configs/windows/vim/.vimrc")
        );
        assert_eq!(
            entry.base_source_for_platform(Platform::Linux),
            Some("configs/shared/vim/.vimrc")
        );
    }

    #[test]
    fn merges_shared_and_platform_fragments() {
        let entry = ConfigEntry {
            id: "zshrc".to_owned(),
            description: None,
            shared: Some("configs/shared/zsh/.zshrc".to_owned()),
            overlays: BTreeMap::new(),
            fragments: vec!["configs/shared/zsh/base.fragment".to_owned()],
            platform_fragments: BTreeMap::from([(
                "android".to_owned(),
                vec!["configs/android/zsh/.zshrc.fragment".to_owned()],
            )]),
            targets: BTreeMap::new(),
        };

        assert_eq!(
            entry.fragments_for_platform(Platform::Linux),
            vec!["configs/shared/zsh/base.fragment"]
        );
        assert_eq!(
            entry.fragments_for_platform(Platform::Android),
            vec![
                "configs/shared/zsh/base.fragment",
                "configs/android/zsh/.zshrc.fragment"
            ]
        );
    }

    #[test]
    fn matches_all_program_entries_without_platforms() {
        let entry = ProgramEntry {
            name: "Firefox".to_owned(),
            url: "https://example.com".to_owned(),
            platforms: Vec::new(),
            notes: None,
        };

        assert!(entry.matches_platform(Platform::Windows));
        assert!(entry.matches_platform(Platform::Linux));
        assert!(entry.matches_platform(Platform::Macos));
        assert!(entry.matches_platform(Platform::Android));
    }
}
