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


use crate::state::ORG_INVITE_SETTINGS;
use rand::{TryRng, rngs::SysRng};
use crate::errors::ServiceResult;
use hmac::{Hmac, Mac, KeyInit};
use sha2::Sha256;


type HmacSha256 = Hmac<Sha256>;



/// Generates a cryptographically secure invite code of the specified length
pub fn generate_invite_code() -> ServiceResult<String> {
    let mut rng = SysRng;   // Initialize the system random number generator

    // Rejection sampling avoids modulo bias
    let upper_bound = u32::MAX - (u32::MAX % ORG_INVITE_SETTINGS.secret_chars_len);

    // Initialize the invite code string with the specified capacity
    let mut invite_code = String::with_capacity(ORG_INVITE_SETTINGS.invite_code_length);

    // Generate each character of the invite code using rejection sampling
    for _ in 0..ORG_INVITE_SETTINGS.invite_code_length {
        // Loop until a valid random index is found
        let index = loop {
            // Requests a random 32-bit number from the system RNG
            let random_number = rng.try_next_u32()?;

            // Check if the random number is within the acceptable range
            // If it is within the acceptable range, it can be used to determine the index
            if random_number < upper_bound {
                break (random_number % ORG_INVITE_SETTINGS.secret_chars_len) as usize;
            }
        };

        // Append the selected character to the invite code string
        invite_code.push(ORG_INVITE_SETTINGS.secret_chars[index]);
    }

    Ok(invite_code)
}


/// Hashes the given invite code using HMAC-SHA256 with a ENV secret key
pub fn hash_invite_code(invite_code: &str) -> String {
    // Initialize the HMAC-SHA256 instance with the secret key
    let mut mac = HmacSha256::new_from_slice(ORG_INVITE_SETTINGS.hash_secret.as_bytes())
        .expect("Invalid HMAC key");

    mac.update(invite_code.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}
