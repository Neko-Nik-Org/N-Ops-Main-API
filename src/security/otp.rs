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


use crate::utils::initial::OTP_CHARSET;
use rand::{TryRng, rngs::SysRng};
use crate::errors::ServiceResult;



/// Generates a cryptographically secure one-time password (OTP) of the specified length
pub fn generate_otp(length: usize) -> ServiceResult<String> {
    // Collect the characters from the allowed character set into a vector
    let chars: Vec<char> = OTP_CHARSET.chars().collect();
    let mut rng = SysRng;   // Initialize the system random number generator

    // Determine the length of the character set
    let charset_len = chars.len() as u32;

    // Rejection sampling avoids modulo bias
    let upper_bound = u32::MAX - (u32::MAX % charset_len);

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
                break (random_number % charset_len) as usize;
            }
        };

        // Append the selected character to the OTP string
        otp.push(chars[index]);
    }

    Ok(otp)
}
