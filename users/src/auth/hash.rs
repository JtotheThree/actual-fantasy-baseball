use argon2::{self, Config};
use rand::{RngCore, rngs::OsRng};

pub fn hash_password(password: &str) -> String {
    let mut r = OsRng::default();
    // Random bytes.
    let mut salt = vec![0u8; 32];
    r.fill_bytes(&mut salt);
    
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &Config::default()).unwrap();
    hash
}

pub fn verify_hash(hash: &str, expected: &str) -> bool {
    argon2::verify_encoded(hash, expected.as_bytes()).unwrap()
}