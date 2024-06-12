use sha2::{Digest, Sha256};

pub fn sha256(to_hash: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(to_hash);
    hasher.finalize().as_slice().to_vec()
}
