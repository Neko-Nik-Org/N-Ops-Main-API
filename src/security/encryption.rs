/*
 * Copyright (C) 2026 Neko Nik <nikhil@nekonik.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 3 along with this program. If not, see
 * <https://www.gnu.org/licenses/>.
 */


use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Nonce,
};
use base64::{
    engine::general_purpose,
    Engine as _,
};



/// Encrypts the given text using AES-256-GCM and returns the base64-encoded ciphertext
pub fn encrypt(key: &[u8], text: &str) -> String {
    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("Invalid encryption key");

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted = cipher
        .encrypt(&nonce, text.as_bytes())
        .expect("encrypt failed");

    let mut data = nonce.to_vec();

    data.extend(encrypted);

    general_purpose::STANDARD.encode(data)
}


/// Decrypts the given base64-encoded ciphertext using AES-256-GCM and returns the original text
pub fn decrypt(key: &[u8], encoded: &str) -> String {
    let data = general_purpose::STANDARD
        .decode(encoded)
        .expect("invalid base64");

    let (nonce_bytes, cipher_bytes) = data.split_at(12);

    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("Invalid encryption key");

    let decrypted = cipher
        .decrypt(
            Nonce::from_slice(nonce_bytes),
            cipher_bytes,
        )
        .expect("decrypt failed");

    String::from_utf8(decrypted)
        .expect("invalid utf8")
}


/// Generate a safe random AES-256 encryption key
pub fn generate_encryption_key() -> String {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    general_purpose::STANDARD.encode(key)
}


/// Convert a base64-encoded encryption key to raw bytes
pub fn decode_encryption_key(encoded: &str) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(encoded)
        .expect("invalid base64")
}
