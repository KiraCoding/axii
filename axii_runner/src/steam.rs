use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs::read_to_string, path::Path};
use vdf_reader::from_str;
use windows_registry::LOCAL_MACHINE;

#[derive(Debug, Deserialize)]
pub(crate) struct Library {
    #[serde(rename = "libraryfolders")]
    folders: Folders,
}

impl Library {
    #[inline(always)]
    pub(crate) fn get() -> Self {
        let steam = LOCAL_MACHINE
            .open("SOFTWARE\\Wow6432Node\\Valve\\Steam")
            .unwrap()
            .get_string("InstallPath")
            .unwrap();

        let steam_path = Path::new(&steam)
            .join("steamapps")
            .join("libraryfolders.vdf");

        let vdf = read_to_string(steam_path).unwrap();

        from_str(&vdf).unwrap()
    }

    #[inline(always)]
    pub(crate) fn install_dir(&self, app_id: &str) -> Option<PathBuf> {
        self.folders.0.values().find_map(|entry| {
            entry.apps.get(app_id).map(|_| {
                let manifest = AppManifest::get(&entry.path, app_id);
                entry
                    .path
                    .join("steamapps")
                    .join("common")
                    .join(manifest.app_state.install_dir)
            })
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Folders(HashMap<u8, Entry>);

#[derive(Debug, Deserialize)]
pub struct Entry {
    path: PathBuf,
    apps: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct AppManifest {
    #[serde(rename = "AppState")]
    app_state: AppState,
}

impl AppManifest {
    #[inline(always)]
    fn get(path: &Path, app_id: &str) -> Self {
        let format = format!("appmanifest_{}.acf", app_id);
        let manifest_path = path.join("steamapps").join(format);
        let manifest = read_to_string(manifest_path).unwrap();

        from_str(&manifest).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct AppState {
    #[serde(rename = "installdir")]
    install_dir: String,
}
