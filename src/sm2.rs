use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct KeyPair(String, String);

#[wasm_bindgen]
impl KeyPair {
    pub fn private_key(&self) -> String {
        self.0.clone()
    }

    pub fn public_key(&self) -> String {
        self.1.clone()
    }
}


#[wasm_bindgen]
pub fn sm2_generate_keypair() -> KeyPair {
    let pair = yarism::sm2::generate_keypair();
    KeyPair(pair.0, pair.1)
}

#[wasm_bindgen]
pub fn sm2_encrypt(public_key: &str, plain: &str) -> String {
    yarism::sm2::encrypt(public_key, plain)
}

#[wasm_bindgen]
pub fn sm2_decrypt(private_key: &str, cipher: &str) -> String {
    yarism::sm2::decrypt(private_key, cipher)
}

#[wasm_bindgen]
pub fn sm2_encrypt_c1c2c3(public_key: &str, plain: &str) -> String {
    yarism::sm2::encrypt_c1c2c3(public_key, plain)
}

#[wasm_bindgen]
pub fn sm2_decrypt_c1c2c3(private_key: &str, cipher: &str) -> String {
    yarism::sm2::decrypt_c1c2c3(private_key, cipher)
}

#[wasm_bindgen]
pub fn sm2_sign(private_key: &str, public_key: &str, plain: &str) -> String {
    yarism::sm2::sign(private_key, public_key, plain)
}

#[wasm_bindgen]
pub fn sm2_verify(public_key: &str, plain: &str, signature: &str) -> bool {
    yarism::sm2::verify(public_key, plain, signature)
}