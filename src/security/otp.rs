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


use rand::{TryRng, rngs::SysRng};
use crate::errors::ServiceResult;
use hmac::{Hmac, Mac, KeyInit};
use crate::state::OTP_SETTINGS;
use sha2::Sha256;


type HmacSha256 = Hmac<Sha256>;



/// Generates a cryptographically secure one-time password (OTP) of the specified length
pub fn generate_otp(length: usize) -> ServiceResult<String> {
    let mut rng = SysRng;   // Initialize the system random number generator

    // Rejection sampling avoids modulo bias
    let upper_bound = u32::MAX - (u32::MAX % OTP_SETTINGS.chars_len);

    // Initialize the OTP string with the specified capacity
    let mut otp = String::with_capacity(length);

    // Generate each character of the OTP using rejection sampling
    for _ in 0..length {
        // Loop until a valid random index is found
        let index = loop {
            // Requests a random 32-bit number from the system RNG
            let random_number = rng.try_next_u32()?;

            // Check if the random number is within the acceptable range
            // If it is within the acceptable range, it can be used to determine the index
            if random_number < upper_bound {
                break (random_number % OTP_SETTINGS.chars_len) as usize;
            }
        };

        // Append the selected character to the OTP string
        otp.push(OTP_SETTINGS.chars[index]);
    }

    Ok(otp)
}


/// Hashes the given one-time password (OTP) using HMAC-SHA256 with a ENV secret key
pub fn hash_otp(otp: &str) -> String {
    // Initialize the HMAC-SHA256 instance with the secret key
    let mut mac = HmacSha256::new_from_slice(OTP_SETTINGS.hash_secret.as_bytes())
        .expect("Invalid HMAC key");

    mac.update(otp.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}
