use crate::{
    app_error::AppError, consts::DEFAULT_CONFIG, serde_file::Serde, util::get_app_directory,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub vaults: Vec<String>,
}

impl ApplicationConfig {
    pub fn load() -> Result<Self, AppError> {
        let mut config: Self = Serde::load(DEFAULT_CONFIG)?;

        let app_directory = get_app_directory()?;

        let mut paths = fs::read_dir(app_directory)?
            .filter(|dir_entry| match dir_entry {
                Ok(dir_entry) => match dir_entry.path().extension() {
                    Some(extension_os_str) => match extension_os_str.to_str() {
                        Some(extension) => extension == "bin",
                        None => false,
                    },
                    None => false,
                },
                Err(_) => false,
            })
            .map(|dir_entry| match dir_entry {
                Ok(dir_entry) => match dir_entry.path().file_stem() {
                    Some(path) => match path.to_str() {
                        Some(path_stem) => path_stem.to_string(),
                        None => unreachable!(),
                    },
                    None => unreachable!(),
                },
                Err(_) => unreachable!(),
            })
            .collect::<Vec<String>>();

        paths.sort();
        config.vaults = paths;

        config.save()?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), AppError> {
        Ok(Serde::save::<Self, &str>(DEFAULT_CONFIG, self)?)
    }
}
