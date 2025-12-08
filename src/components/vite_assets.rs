use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ViteManifest {
    #[serde(flatten)]
    entries: HashMap<String, ViteManifestEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ViteManifestEntry {
    file: String,
    #[serde(default)]
    css: Vec<String>,
    #[serde(default)]
    assets: Vec<String>,
}

#[derive(Default, Debug, Clone)]
pub struct ViteAssets {
    manifest: Option<ViteManifest>,
}

impl ViteAssets {
    pub fn new() -> Self {
        let manifest = match cfg!(debug_assertions) {
            true => None,
            false => Self::load_manifest(),
        };

        Self { manifest }
    }

    fn load_manifest() -> Option<ViteManifest> {
        let manifest_path = Path::new("public/assets/.vite/manifest.json");
        let content = fs::read_to_string(manifest_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn get_asset_url(&self, entry: &str) -> String {
        if cfg!(debug_assertions) {
            return format!("http://localhost:5173/assets/src/{entry}");
        }

        if let Some(manifest) = &self.manifest
            && let Some(entry_data) = manifest.entries.get(&format!("src/{entry}"))
        {
            return format!("/assets/{}", entry_data.file);
        }

        String::new()
    }

    pub fn get_css_urls(&self, entry: &str) -> Vec<String> {
        if cfg!(debug_assertions) {
            return Vec::new();
        }

        if let Some(manifest) = &self.manifest
            && let Some(entry_data) = manifest.entries.get(&format!("src/{entry}"))
        {
            return entry_data
                .css
                .iter()
                .map(|css| format!("/assets/{css}"))
                .collect();
        }

        Vec::new()
    }

    pub fn get_vite_client(&self) -> Option<String> {
        match cfg!(debug_assertions) {
            true => Some("http://localhost:5173/assets/@vite/client".to_owned()),
            false => None,
        }
    }
}
