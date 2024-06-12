use crate::{
    app_error::AppError,
    encryption::{decrypt::decrypt, encrypt::encrypt},
    util::get_file_path,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{
    ffi::OsStr,
    fs::{read, write},
    io::ErrorKind,
};

pub struct SerdeEncrypted;

impl SerdeEncrypted {
    pub fn load<Type, File>(file: File, password: String) -> Result<Type, AppError>
    where
        Type: for<'a> Deserialize<'a> + Serialize + Default,
        File: Into<String> + AsRef<OsStr>,
    {
        let file_path = get_file_path(&file)?;

        let data = match read(file_path) {
            Ok(data_source) => from_str::<Type>(&decrypt(password, data_source)?)
                .map_err(|_| AppError::DeserializationFailed)?,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                let default_data = Type::default();

                Self::save::<Type, File>(file, &default_data, password)?;

                default_data
            }
            Err(error) => return Err(error.into()),
        };

        Ok(data)
    }

    pub fn save<Type, File>(file: File, data: &Type, password: String) -> Result<(), AppError>
    where
        Type: for<'a> Deserialize<'a> + Serialize + Default,
        File: Into<String> + AsRef<OsStr>,
    {
        let file_path = get_file_path(&file)?;

        let serialized_data = encrypt(
            password,
            to_string_pretty(data).map_err(|_| AppError::SerializationFailed)?,
        )?;

        write(file_path, serialized_data)?;

        Ok(())
    }
}
