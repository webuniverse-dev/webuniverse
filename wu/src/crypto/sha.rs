//! SHA3-256 hashing

pub use hex::{decode as hex_decode, encode as hex_encode};
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(webuniverse + sha3-256(username + sha3-256(password))
pub fn hash_password(password: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let mut enc = hex_encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    enc = hex_encode(hasher.result());

    // hash the hash with webuniverse
    hasher = Sha3_256::new();
    hasher.input(b"webuniverse");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    hex_encode(result)
}

/// SHA3-256 Hash
pub fn hash(plaintext: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(plaintext);
    hex_encode(hasher.result())
}