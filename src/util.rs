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
