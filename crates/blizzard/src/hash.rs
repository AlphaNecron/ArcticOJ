use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash};

fn default_conf<'a>() -> Argon2<'a> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(1 << 16, 3, 4, None).unwrap(),
    )
}

pub fn hash_pwd(pwd: &str) -> String {
    let conf = default_conf();
    let salt = password_hash::SaltString::generate();
    conf.hash_password(pwd.as_bytes(), &salt)
        .map(|h| h.to_string())
        .expect("error hashing password")
}

pub fn verify_pwd(hash: &str, pwd: &str) -> bool {
    let conf = default_conf();
    let ph = password_hash::PasswordHash::new(hash).expect("error parsing hash");
    conf.verify_password(pwd.as_bytes(), &ph).is_ok()
}
