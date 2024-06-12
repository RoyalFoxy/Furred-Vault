use crate::app_error::AppError;
use crate::encryption::hashing::sha256;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};

pub fn decrypt(password: String, mut encrypted_data: Vec<u8>) -> Result<String, AppError> {
    let mut key = [0u8; 32];

    let hashed_password = sha256(&password);

    key.copy_from_slice(&hashed_password[..32]);

    let nonce_length = encrypted_data.pop().ok_or(AppError::AesFailedDecryption)?;
    let nonce_bytes = encrypted_data
        .iter()
        .rev()
        .take(nonce_length as usize)
        .rev()
        .cloned()
        .collect::<Vec<u8>>();
    let cipher_text = &encrypted_data[..encrypted_data.len() - nonce_length as usize];

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, cipher_text.as_ref())
        .map_err(|_| AppError::AesFailedDecryption)?;

    Ok(std::str::from_utf8(&plaintext)?.to_owned())
}
