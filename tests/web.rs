//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn wasm_sm3_digest() {
    let hash = wsm4w::sm3::sm3_digest("abc");
    assert_eq!(hash, "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0");
}


#[wasm_bindgen_test]
fn wasm_sm2_encrypt_decrypt() {
    let pair = wsm4w::sm2::sm2_generate_keypair();
    let plain = String::from("兽人永不为奴，我们终将成王。——加尔鲁什·地狱咆哮");
    let cipher = wsm4w::sm2::sm2_encrypt(&pair.public_key(), &plain);
    let text = wsm4w::sm2::sm2_decrypt(&pair.private_key(), &cipher);
    assert_eq!(plain, text);
}


#[wasm_bindgen_test]
fn wasm_sm2_encrypt_decrypt_c1c2c3() {
    let pair = wsm4w::sm2::sm2_generate_keypair();
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let cipher = wsm4w::sm2::sm2_encrypt_c1c2c3(&pair.public_key(), plain);
    let text = wsm4w::sm2::sm2_decrypt_c1c2c3(&pair.private_key(), &cipher);
    assert_eq!(plain, text);
}


#[wasm_bindgen_test]
fn wasm_sm2_sign_verify() {
    let pair = wsm4w::sm2::sm2_generate_keypair();
    let (prk, puk) = (pair.private_key(), pair.public_key());
    let plain = "没人生来杰出（No one breather who is worthier）——奥格瑞姆·毁灭之锤";
    let signature = wsm4w::sm2::sm2_sign(&prk, &puk, plain);
    let flag = wsm4w::sm2::sm2_verify(&puk, plain, &signature);
    assert_eq!(flag, true);
}

#[wasm_bindgen_test]
fn wasm_sm4_encrypt_decrypt_ecb() {
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let key = wsm4w::sm4::sm4_generate_key();
    let cipher = wsm4w::sm4::sm4_encrypt_ecb(key.clone(), String::from(plain));
    let text = wsm4w::sm4::sm4_decrypt_ecb(key.clone(), cipher);
    assert_eq!(plain, text);
}

#[wasm_bindgen_test]
fn wasm_sm4_encrypt_decrypt_cbc() {
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let key = wsm4w::sm4::sm4_generate_key();
    let iv = wsm4w::sm4::sm4_generate_iv();
    let cipher = wsm4w::sm4::sm4_encrypt_cbc(key.clone(), iv.clone(), String::from(plain));
    let text = wsm4w::sm4::sm4_decrypt_cbc(key.clone(), iv.clone(), cipher);
    assert_eq!(plain, text);
}

#[wasm_bindgen_test]
fn wasm_sm4_encrypt_decrypt_cfb() {
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let key = wsm4w::sm4::sm4_generate_key();
    let iv = wsm4w::sm4::sm4_generate_iv();
    let cipher = wsm4w::sm4::sm4_encrypt_cfb(key.clone(), iv.clone(), String::from(plain));
    let text = wsm4w::sm4::sm4_decrypt_cfb(key.clone(), iv.clone(), cipher);
    assert_eq!(plain, text);
}

#[wasm_bindgen_test]
fn wasm_sm4_encrypt_decrypt_ofb() {
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let key = wsm4w::sm4::sm4_generate_key();
    let iv = wsm4w::sm4::sm4_generate_iv();
    let cipher = wsm4w::sm4::sm4_encrypt_ofb(key.clone(), iv.clone(), String::from(plain));
    let text = wsm4w::sm4::sm4_decrypt_ofb(key.clone(), iv.clone(), cipher);
    assert_eq!(plain, text);
}

#[wasm_bindgen_test]
fn wasm_sm4_encrypt_decrypt_ctr() {
    let plain = "圣光会抛弃你的，英雄，就像抛弃我那样。——巫妖王";
    let key = wsm4w::sm4::sm4_generate_key();
    let iv = wsm4w::sm4::sm4_generate_iv();
    let cipher = wsm4w::sm4::sm4_encrypt_ctr(key.clone(), iv.clone(), String::from(plain));
    let text = wsm4w::sm4::sm4_decrypt_ctr(key.clone(), iv.clone(), cipher);
    assert_eq!(plain, text);
}