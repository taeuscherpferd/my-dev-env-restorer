use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

pub fn discover_repo_root(start: &Path) -> Result<PathBuf> {
    let mut current = start.canonicalize().with_context(|| {
        format!(
            "Failed to resolve the current working directory {}",
            start.display()
        )
    })?;

    loop {
        if current.join("manifests").join("configs.toml").exists() {
            return Ok(current);
        }

        if !current.pop() {
            break;
        }
    }

    Err(anyhow!(
        "Could not find the repository root. Run the command from this repo or one of its subdirectories."
    ))
}

pub fn expand_target_path(raw: &str) -> Result<PathBuf> {
    let with_home = expand_home(raw)?;
    let expanded = expand_windows_style_env(&with_home)?;

    Ok(PathBuf::from(expanded))
}

fn expand_home(raw: &str) -> Result<String> {
    if let Some(stripped) = raw.strip_prefix("~/") {
        return Ok(format!(
            "{}{}{}",
            home_dir()?,
            std::path::MAIN_SEPARATOR,
            stripped
        ));
    }

    if raw == "~" {
        return Ok(home_dir()?);
    }

    Ok(raw.to_owned())
}

fn home_dir() -> Result<String> {
    env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| anyhow!("Could not determine the current user's home directory"))
}

fn expand_windows_style_env(raw: &str) -> Result<String> {
    let mut expanded = String::with_capacity(raw.len());
    let mut remainder = raw;

    while let Some(start_index) = remainder.find('%') {
        expanded.push_str(&remainder[..start_index]);

        let after_start = &remainder[start_index + 1..];
        let Some(end_index) = after_start.find('%') else {
            expanded.push_str(&remainder[start_index..]);
            return Ok(expanded);
        };

        let key = &after_start[..end_index];
        let value = env::var(key).with_context(|| {
            format!("Missing environment variable %{key}% while expanding {raw}")
        })?;

        expanded.push_str(&value);
        remainder = &after_start[end_index + 1..];
    }

    expanded.push_str(remainder);
    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::expand_home;
    use super::expand_windows_style_env;

    #[test]
    fn leaves_plain_paths_unchanged() {
        let path = expand_home("plain/path").expect("path should expand");
        assert_eq!(path, "plain/path");
    }

    #[test]
    fn keeps_incomplete_windows_placeholder() {
        let path = expand_windows_style_env("%USERPROFILE").expect("path should expand");
        assert_eq!(path, "%USERPROFILE");
    }
}
