use crate::{app_error::AppError, consts::APP_NAME};
use directories::ProjectDirs;
use std::{fs::create_dir_all, path::PathBuf};

pub fn get_file_path<File>(file: File) -> Result<PathBuf, AppError>
where
    File: Into<PathBuf>,
{
    let app_directory = get_app_directory()?;

    Ok([app_directory, file.into()].iter().collect())
}

pub fn get_app_directory() -> Result<PathBuf, AppError> {
    let app_directory = ProjectDirs::from("", "", APP_NAME)
        .ok_or_else(|| AppError::DeterminingHomeDirectoryFailed)?;

    let app_directory_path: PathBuf = app_directory.config_dir().into();

    if !app_directory_path.exists() {
        create_dir_all(&app_directory_path)?;
    }

    Ok(app_directory_path)
}
