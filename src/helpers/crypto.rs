use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305};
use argon2::{self, Config, ThreadMode, Variant, Version};
use generic_array::GenericArray;

/// Hash data in bytes using Argon2id
pub fn hash(data: &[u8]) -> String {
    argon2::hash_encoded(
        data,
        super::random_string(16).as_bytes(),
        &Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 524288,
            time_cost: 1,
            lanes: 8,
            thread_mode: ThreadMode::Parallel,
            secret: dotenv::var("KEY").expect("Missing env `KEY`").as_bytes(),
            ad: &[],
            hash_length: 32
        }
    ).unwrap()
}

/// Test if the password is corresponding with another one hashed
pub fn hash_test(hash: &str, pwd: &[u8]) -> bool {
    argon2::verify_encoded_ext(hash, pwd, dotenv::var("KEY").expect("Missing env `KEY`").as_bytes(), &[]).unwrap_or(false)
}

/// Encrypt data as bytes into String with ChaCha20 (Salsa20) and Poly1305 
pub fn encrypt(data: &[u8]) -> String {
    match hex::decode(dotenv::var("CHA_KEY").expect("Missing env `CHA_KEY`")) {
        Ok(v) => {
            let bytes = GenericArray::clone_from_slice(&v);

            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
            match ChaCha20Poly1305::new(&bytes).encrypt(&nonce, data) {
                Ok(y) => format!("{}//{}", hex::encode(nonce), hex::encode(y)),
                Err(_) => "Error".to_string(),
            }
        },
        Err(_) => "Error".to_string(),
    }
}

/// Decrypt a string with ChaCha20 (Salsa20) and Poly1305
pub fn decrypt(data: String) -> String {
    let splited = data.split_once("//").unwrap_or_else(|| ("", ""));

    match hex::decode(dotenv::var("CHA_KEY").expect("Missing env `CHA_KEY`")) {
        Ok(v) => {
            let bytes = GenericArray::clone_from_slice(&v);
            match hex::decode(splited.0) {
                Ok(x) => {
                    let arr_ref = GenericArray::from_slice(&x);

                    match ChaCha20Poly1305::new(&bytes).decrypt(arr_ref, hex::decode(splited.1).unwrap().as_ref()) {
                        Ok(y) => String::from_utf8(y).unwrap(),
                        Err(_) => "Error".to_string(),
                    }
                },
                Err(_) => "Error".to_string(),
            }
        },
        Err(_) => "Error".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let pwd = &hash(b"password");
        assert!(regex::Regex::new(r"[$]argon2(i)?(d)?[$]v=[0-9]{1,2}[$]m=[0-9]+,t=[0-9]{1,},p=[0-9]{1,}[$].*").unwrap().is_match(pwd));
        assert!(hash_test(pwd, b"password"));
    }
}
