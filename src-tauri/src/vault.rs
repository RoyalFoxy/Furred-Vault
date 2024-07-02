use crate::{app_error::AppError, serde_file_encryption::SerdeEncrypted};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PasswordEntry {
    id: Uuid,
    username: String,
    password: String,
    website: String,
    notes: String,
    tags: Vec<String>,
}

impl PasswordEntry {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn website(&self) -> String {
        self.website.clone()
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VaultData {
    name: String,
    pub passwords: Vec<PasswordEntry>,
}

impl VaultData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            passwords: Vec::new(),
        }
    }

    pub fn load_from_file(vault: String, password: String) -> Result<Self, AppError> {
        Ok(SerdeEncrypted::load::<Self, _>(
            Self::file_name(vault),
            password,
        )?)
    }

    pub fn save_to_file(&self, password: String) -> Result<(), AppError> {
        SerdeEncrypted::save(Self::file_name(self.name.to_string()), self, password)?;

        Ok(())
    }

    pub fn file_name(name: impl Into<String>) -> String {
        format!("{}.bin", name.into())
    }

    pub fn rename(&mut self, name: String) {
        self.name = name
    }
}
