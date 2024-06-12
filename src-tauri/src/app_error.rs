use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Something unexpected happened")]
    InternalServerError,

    #[error("Vault already exists")]
    VaultAlreadyExists,

    #[error("Vault does not exist")]
    VaultDoesNotExist,

    #[error("Getting parent of path failed")]
    GettingParentFailed,

    #[error("Decryption failed")]
    AesFailedDecryption,

    #[error("Encryption failed")]
    AesFailedEncryption,

    #[error("Password deletion failed")]
    PasswordDeletionFailed,

    #[error("Password updating failed")]
    PasswordUpdatingFailed,

    #[error("Determining home directory failed")]
    DeterminingHomeDirectoryFailed,

    #[error("Converting app directory failed")]
    ConvertingAppDirectoryFailed,

    #[error("Deserialization failed")]
    DeserializationFailed,

    #[error("Serialization failed")]
    SerializationFailed,

    #[error("Initialization of history failed")]
    HistoryInitFailed,

    #[error("{0}")]
    String(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
