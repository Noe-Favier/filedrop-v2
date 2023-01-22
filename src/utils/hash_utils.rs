extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash as bcrypt_hash, verify, BcryptResult};

pub fn hash(str: String) -> BcryptResult<String>{
    return bcrypt_hash(str, DEFAULT_COST);
}

pub fn check(str: String, hash: &str) -> bool{
    let result = verify(str, hash);
    if result.is_ok() {
        return result.unwrap();
    }
    return false;
}