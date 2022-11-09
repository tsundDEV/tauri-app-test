pub mod constants;

use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::api::path::data_dir;

// On every new window run init
pub fn init() {
    // Create the root dir
    let root_dir: PathBuf = get_config_path();
    fs::create_dir_all(root_dir).unwrap();
}

// Return config file
pub fn get_config_path() -> PathBuf {
    return Path::new(&data_dir().unwrap()).join(format!(
        "{}/{}",
        constants::LOCAL_DIR_NAME,
        constants::CONFIG_DIR_NAME
    ));
}
