use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sm4_generate_key() -> String {
    yarism::sm4::generate_key()
}

#[wasm_bindgen]
pub fn sm4_generate_iv() -> String {
    yarism::sm4::generate_iv()
}

#[wasm_bindgen]
pub fn sm4_encrypt_ecb(key: String, plain: String) -> String {
    yarism::sm4::encrypt_ecb(key, plain)
}

#[wasm_bindgen]
pub fn sm4_decrypt_ecb(key: String, cipher: String) -> String {
    yarism::sm4::decrypt_ecb(key, cipher)
}

#[wasm_bindgen]
pub fn sm4_encrypt_cbc(key: String, iv: String, plain: String) -> String {
    yarism::sm4::encrypt_cbc(key, iv, plain)
}

#[wasm_bindgen]
pub fn sm4_decrypt_cbc(key: String, iv: String, cipher: String) -> String {
    yarism::sm4::decrypt_cbc(key, iv, cipher)
}

#[wasm_bindgen]
pub fn sm4_encrypt_cfb(key: String, iv: String, plain: String) -> String {
    yarism::sm4::encrypt_cfb(key, iv, plain)
}

#[wasm_bindgen]
pub fn sm4_decrypt_cfb(key: String, iv: String, cipher: String) -> String {
    yarism::sm4::decrypt_cfb(key, iv, cipher)
}

#[wasm_bindgen]
pub fn sm4_encrypt_ofb(key: String, iv: String, plain: String) -> String {
    yarism::sm4::encrypt_ofb(key, iv, plain)
}

#[wasm_bindgen]
pub fn sm4_decrypt_ofb(key: String, iv: String, cipher: String) -> String {
    yarism::sm4::decrypt_ofb(key, iv, cipher)
}

#[wasm_bindgen]
pub fn sm4_encrypt_ctr(key: String, iv: String, plain: String) -> String {
    yarism::sm4::encrypt_ctr(key, iv, plain)
}

#[wasm_bindgen]
pub fn sm4_decrypt_ctr(key: String, iv: String, cipher: String) -> String {
    yarism::sm4::decrypt_ctr(key, iv, cipher)
}
