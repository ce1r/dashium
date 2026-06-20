use sha1::Digest;
use sha1::Sha1;

pub fn is_ascii_alphanumeric(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn salt_and_sha1(input: &str, salt: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn cyclic_xor(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}
