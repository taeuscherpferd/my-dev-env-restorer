use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, anyhow};

use crate::cli::Cli;
use crate::manifest::{
    ConfigEntry, ConfigTarget, Platform, ProgramEntry, load_config_manifest, load_program_manifest,
};
use crate::repo::{discover_repo_root, expand_target_path};

pub fn run(cli: Cli) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to read the current directory")?;
    let repo_root = discover_repo_root(&current_dir)?;
    let host_platform = Platform::current();

    if cli.configs {
        sync_configs(
            &repo_root,
            host_platform,
            &cli.config_ids,
            &cli.targets,
            cli.all_targets,
            SyncDirection::Apply,
            cli.dry_run,
        )?;
    }

    if cli.pull {
        sync_configs(
            &repo_root,
            host_platform,
            &cli.config_ids,
            &cli.targets,
            cli.all_targets,
            SyncDirection::Pull,
            cli.dry_run,
        )?;
    }

    if cli.links {
        open_program_links(&repo_root, host_platform, cli.dry_run)?;
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
    host_platform: Platform,
    selected_config_ids: &[String],
    selected_targets: &[String],
    all_targets: bool,
    direction: SyncDirection,
    dry_run: bool,
) -> Result<()> {
    let manifest_path = repo_root.join("manifests").join("configs.toml");
    let manifest = load_config_manifest(&manifest_path)?;
    validate_selected_config_ids(&manifest.config, selected_config_ids)?;

    let mut changed = 0usize;
    let mut skipped = 0usize;
    let mut matched_target_names = BTreeSet::new();

    for entry in &manifest.config {
        if !selected_config_ids.is_empty()
            && !selected_config_ids
                .iter()
                .any(|config_id| config_id == &entry.id)
        {
            continue;
        }

        let targets = entry.selectable_targets(host_platform, selected_targets, all_targets);

        if targets.is_empty() {
            skipped += 1;
            continue;
        }

        for target in targets {
            matched_target_names.insert(target.name.clone());
            let machine_path = expand_target_path(&target.path)?;

            match direction {
                SyncDirection::Apply => {
                    let generated = build_repo_output(repo_root, entry, target)?;
                    write_generated_file(&generated, &machine_path, entry, target, dry_run)?;
                    changed += 1;
                }
                SyncDirection::Pull => {
                    if entry.is_generated_for_target(target) {
                        println!(
                            "skip {} [{}]: generated from shared base plus fragment overlays; pull is not automatic",
                            entry.id, target.name
                        );
                        skipped += 1;
                        continue;
                    }

                    let Some(target_platform) = target.platform_enum() else {
                        return Err(anyhow!(
                            "Config '{}' target '{}' uses an unsupported platform '{}'",
                            entry.id,
                            target.name,
                            target.platform
                        ));
                    };

                    let Some(source_relative) = entry.base_source_for_platform(target_platform)
                    else {
                        return Err(anyhow!(
                            "Config '{}' has no source for target '{}' on platform {}",
                            entry.id,
                            target.name,
                            target.platform
                        ));
                    };

                    let repo_path = repo_root.join(source_relative);

                    if !machine_path.exists() {
                        println!(
                            "skip {} [{}]: target does not exist at {}",
                            entry.id,
                            target.name,
                            machine_path.display()
                        );
                        skipped += 1;
                        continue;
                    }

                    copy_file(
                        &machine_path,
                        &repo_path,
                        entry,
                        target,
                        "machine",
                        "repo",
                        dry_run,
                    )?;
                    changed += 1;
                }
            }
        }
    }

    if !selected_targets.is_empty() {
        for requested_target in selected_targets {
            if !matched_target_names.contains(requested_target) {
                return Err(anyhow!("Unknown target '{}'.", requested_target));
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

fn validate_selected_config_ids(
    entries: &[ConfigEntry],
    selected_config_ids: &[String],
) -> Result<()> {
    if selected_config_ids.is_empty() {
        return Ok(());
    }

    let known_config_ids = entries
        .iter()
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();

    for requested_config_id in selected_config_ids {
        if !known_config_ids.contains(requested_config_id.as_str()) {
            return Err(anyhow!("Unknown config id '{}'.", requested_config_id));
        }
    }

    Ok(())
}

fn build_repo_output(
    repo_root: &Path,
    entry: &ConfigEntry,
    target: &ConfigTarget,
) -> Result<String> {
    let Some(target_platform) = target.platform_enum() else {
        return Err(anyhow!(
            "Config '{}' target '{}' uses an unsupported platform '{}'",
            entry.id,
            target.name,
            target.platform
        ));
    };

    let Some(base_relative) = entry.base_source_for_platform(target_platform) else {
        return Err(anyhow!(
            "Config '{}' has no source for target '{}' on platform {}",
            entry.id,
            target.name,
            target.platform
        ));
    };

    let mut output = read_repo_file(&repo_root.join(base_relative), entry, "base source")?;

    for fragment_relative in entry.fragments_for_target(target) {
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
    target: &ConfigTarget,
    dry_run: bool,
) -> Result<()> {
    let description = entry
        .description
        .as_deref()
        .unwrap_or("no description provided");

    if dry_run {
        println!(
            "dry-run {} [{}]: generated output -> {} ({description})",
            entry.id,
            target.name,
            destination.display()
        );
        return Ok(());
    }

    let parent = destination.parent().ok_or_else(|| {
        anyhow!(
            "Config '{}' target '{}' resolved to a destination without a parent directory: {}",
            entry.id,
            target.name,
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
            "Failed to write generated config '{}' target '{}' to {}",
            entry.id,
            target.name,
            destination.display()
        )
    })?;

    println!(
        "generated {} [{}] -> {}",
        entry.id,
        target.name,
        destination.display()
    );
    println!("synced to machine: {description}");
    Ok(())
}

fn copy_file(
    source: &Path,
    destination: &Path,
    entry: &ConfigEntry,
    target: &ConfigTarget,
    source_label: &str,
    destination_label: &str,
    dry_run: bool,
) -> Result<()> {
    if !source.exists() {
        return Err(anyhow!(
            "Config '{}' target '{}' expected a source file at {}",
            entry.id,
            target.name,
            source.display()
        ));
    }

    let description = entry
        .description
        .as_deref()
        .unwrap_or("no description provided");

    if dry_run {
        println!(
            "dry-run {} [{}]: {} -> {} ({description})",
            entry.id,
            target.name,
            source.display(),
            destination.display()
        );
        return Ok(());
    }

    let parent = destination.parent().ok_or_else(|| {
        anyhow!(
            "Config '{}' target '{}' resolved to a destination without a parent directory: {}",
            entry.id,
            target.name,
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
            "Failed to copy config '{}' target '{}' from {} to {}",
            entry.id,
            target.name,
            source.display(),
            destination.display()
        )
    })?;

    println!(
        "{} {} [{}]: {} -> {}",
        source_label,
        entry.id,
        target.name,
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
    use crate::manifest::{ConfigEntry, ConfigTarget, Platform, ProgramEntry};
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
            target_fragments: BTreeMap::new(),
            targets: Vec::new(),
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
    fn merges_shared_platform_and_target_fragments() {
        let entry = ConfigEntry {
            id: "zshrc".to_owned(),
            description: None,
            shared: Some("configs/shared/zsh/.zshrc".to_owned()),
            overlays: BTreeMap::new(),
            fragments: vec!["configs/shared/zsh/base.fragment".to_owned()],
            platform_fragments: BTreeMap::from([(
                "linux".to_owned(),
                vec!["configs/linux/zsh/linux.fragment".to_owned()],
            )]),
            target_fragments: BTreeMap::from([(
                "windows-wsl-main".to_owned(),
                vec!["configs/wsl/zsh/target.fragment".to_owned()],
            )]),
            targets: Vec::new(),
        };
        let target = ConfigTarget {
            name: "windows-wsl-main".to_owned(),
            platform: "linux".to_owned(),
            path: "/tmp/.zshrc".to_owned(),
            default: false,
            hosts: vec!["windows".to_owned()],
        };

        assert_eq!(
            entry.fragments_for_target(&target),
            vec![
                "configs/shared/zsh/base.fragment",
                "configs/linux/zsh/linux.fragment",
                "configs/wsl/zsh/target.fragment"
            ]
        );
    }

    #[test]
    fn selects_default_targets_for_host_platform() {
        let entry = ConfigEntry {
            id: "vimrc".to_owned(),
            description: None,
            shared: Some("configs/shared/vim/.vimrc".to_owned()),
            overlays: BTreeMap::new(),
            fragments: Vec::new(),
            platform_fragments: BTreeMap::new(),
            target_fragments: BTreeMap::new(),
            targets: vec![
                ConfigTarget {
                    name: "windows-host".to_owned(),
                    platform: "windows".to_owned(),
                    path: "a".to_owned(),
                    default: true,
                    hosts: Vec::new(),
                },
                ConfigTarget {
                    name: "windows-wsl-main".to_owned(),
                    platform: "linux".to_owned(),
                    path: "b".to_owned(),
                    default: false,
                    hosts: vec!["windows".to_owned()],
                },
            ],
        };

        let selected = entry.selectable_targets(Platform::Windows, &[], false);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].name, "windows-host");

        let all_selected = entry.selectable_targets(Platform::Windows, &[], true);
        assert_eq!(all_selected.len(), 2);
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

    #[test]
    fn accepts_known_selected_config_ids() {
        let entries = vec![ConfigEntry {
            id: "agents".to_owned(),
            description: None,
            shared: Some("configs/shared/AI/AGENTS.md".to_owned()),
            overlays: BTreeMap::new(),
            fragments: Vec::new(),
            platform_fragments: BTreeMap::new(),
            target_fragments: BTreeMap::new(),
            targets: Vec::new(),
        }];
        let selected_config_ids = vec!["agents".to_owned()];

        super::validate_selected_config_ids(&entries, &selected_config_ids)
            .expect("known config ids should be accepted");
    }

    #[test]
    fn rejects_unknown_selected_config_ids() {
        let entries = vec![ConfigEntry {
            id: "agents".to_owned(),
            description: None,
            shared: Some("configs/shared/AI/AGENTS.md".to_owned()),
            overlays: BTreeMap::new(),
            fragments: Vec::new(),
            platform_fragments: BTreeMap::new(),
            target_fragments: BTreeMap::new(),
            targets: Vec::new(),
        }];
        let selected_config_ids = vec!["agnets".to_owned()];

        let result = super::validate_selected_config_ids(&entries, &selected_config_ids);

        assert!(result.is_err());
    }
}
