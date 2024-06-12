use crate::{app_error::AppError, encryption::hashing::sha256};
use aes::cipher::generic_array::GenericArray;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use rand::{rngs::OsRng, RngCore};

pub fn encrypt(password: String, data: String) -> Result<Vec<u8>, AppError> {
    const NONCE_LENGTH: u8 = 12;

    let mut key = [0u8; 32];

    let hashed_password = sha256(&password);

    key.copy_from_slice(&hashed_password[..32]);

    let mut nonce_bytes = [0u8; NONCE_LENGTH as usize];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    let cipher_text = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|_| AppError::AesFailedEncryption)?;

    let mut result = cipher_text;
    result.extend(nonce_bytes);
    result.push(NONCE_LENGTH as u8);

    Ok(result)
}
