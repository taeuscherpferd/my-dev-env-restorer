use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    Macos,
    Android,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "android") {
            Self::Android
        } else if cfg!(target_os = "macos") {
            Self::Macos
        } else {
            Self::Linux
        }
    }

    pub fn as_key(self) -> &'static str {
        match self {
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Android => "android",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigManifest {
    #[serde(default)]
    pub config: Vec<ConfigEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigEntry {
    pub id: String,
    pub description: Option<String>,
    pub shared: Option<String>,
    #[serde(default)]
    pub overlays: BTreeMap<String, String>,
    #[serde(default)]
    pub fragments: Vec<String>,
    #[serde(default)]
    pub platform_fragments: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub target_fragments: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub targets: Vec<ConfigTarget>,
}

impl ConfigEntry {
    pub fn base_source_for_platform(&self, platform: Platform) -> Option<&str> {
        self.overlays
            .get(platform.as_key())
            .map(String::as_str)
            .or(self.shared.as_deref())
    }

    pub fn fragments_for_target(&self, target: &ConfigTarget) -> Vec<&str> {
        let mut fragments = self
            .fragments
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();

        if let Some(platform_fragments) = self.platform_fragments.get(target.platform.as_str()) {
            fragments.extend(platform_fragments.iter().map(String::as_str));
        }

        if let Some(target_fragments) = self.target_fragments.get(target.name.as_str()) {
            fragments.extend(target_fragments.iter().map(String::as_str));
        }

        fragments
    }

    pub fn is_generated_for_target(&self, target: &ConfigTarget) -> bool {
        !self.fragments_for_target(target).is_empty()
    }

    pub fn selectable_targets<'a>(
        &'a self,
        host_platform: Platform,
        selected_targets: &[String],
        all_targets: bool,
    ) -> Vec<&'a ConfigTarget> {
        if !selected_targets.is_empty() {
            return self
                .targets
                .iter()
                .filter(|target| selected_targets.iter().any(|name| name == &target.name))
                .collect();
        }

        if all_targets {
            return self
                .targets
                .iter()
                .filter(|target| target.is_manageable_from(host_platform))
                .collect();
        }

        self.targets
            .iter()
            .filter(|target| target.default && target.is_manageable_from(host_platform))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigTarget {
    pub name: String,
    pub platform: String,
    pub path: String,
    #[serde(default = "default_true")]
    pub default: bool,
    #[serde(default)]
    pub hosts: Vec<String>,
}

impl ConfigTarget {
    pub fn is_manageable_from(&self, host_platform: Platform) -> bool {
        if self.hosts.is_empty() {
            return self.platform == host_platform.as_key();
        }

        self.hosts.iter().any(|host| {
            let normalized = host.trim().to_ascii_lowercase();
            normalized == "all" || normalized == host_platform.as_key()
        })
    }

    pub fn platform_enum(&self) -> Option<Platform> {
        match self.platform.as_str() {
            "windows" => Some(Platform::Windows),
            "linux" => Some(Platform::Linux),
            "macos" => Some(Platform::Macos),
            "android" => Some(Platform::Android),
            _ => None,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct ProgramManifest {
    #[serde(default)]
    pub program: Vec<ProgramEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ProgramEntry {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub platforms: Vec<String>,
    pub notes: Option<String>,
}

impl ProgramEntry {
    pub fn matches_platform(&self, platform: Platform) -> bool {
        self.platforms.is_empty()
            || self.platforms.iter().any(|value| {
                let normalized = value.trim().to_ascii_lowercase();
                normalized == "all" || normalized == platform.as_key()
            })
    }
}

pub fn load_config_manifest(path: &Path) -> Result<ConfigManifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config manifest at {}", path.display()))?;

    toml::from_str(&content)
        .with_context(|| format!("Failed to parse config manifest at {}", path.display()))
}

pub fn load_program_manifest(path: &Path) -> Result<ProgramManifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read program manifest at {}", path.display()))?;

    toml::from_str(&content)
        .with_context(|| format!("Failed to parse program manifest at {}", path.display()))
}
