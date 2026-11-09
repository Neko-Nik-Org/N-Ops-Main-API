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


use base64::{engine::general_purpose::STANDARD, Engine as _};
use cidr::{Ipv4Cidr, Ipv6Cidr};
use std::net::IpAddr;



/// Decodes a base64-encoded password into a Vec<u8>
fn decode_base64_password(base64_password: &str) -> Vec<u8> {
    STANDARD.decode(base64_password).unwrap_or_default()
}


/// Check if a client IP is allowed based on a given IP range (CIDR notation)
/// Returns true if the client IP is within the specified range, false otherwise
fn check_ip_allowed(client_ip: &str, ip_range: &str) -> bool {
    // Parse the client IP address into an Ipv4Addr
    let client_ip: IpAddr = match client_ip.parse() {
        Ok(ip) => ip,
        Err(_) => return false,
    };

    // Parse the IP range into an Ipv4Cidr and check if the client IP is within it
    if let Ok(v4_range) = ip_range.parse::<Ipv4Cidr>() {
        if let IpAddr::V4(v4) = client_ip {
            if v4_range.contains(&v4) {
                return true;
            }
        }
    }

    // Parse the IP range into an Ipv6Cidr and check if the client IP is within it
    if let Ok(v6_range) = ip_range.parse::<Ipv6Cidr>() {
        if let IpAddr::V6(v6) = client_ip {
            if v6_range.contains(&v6) {
                return true;
            }
        }
    }

    false   // Invalid CIDR string
}


/// Generates a password hash from a given password string
pub fn generate_password_hash(password: &str) -> String {
    // Implementation for generating a password hash goes here
    // For example, using bcrypt:
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}


/// Verifies a base64-encoded password against a given hash
pub fn verify_password_hash(base64_password: &str, hash: &str) -> bool {
    let password = decode_base64_password(base64_password);

    bcrypt::verify(password, hash).unwrap_or(false)
}


/// Generates a 6-digit one-time password (OTP) as a string
pub fn generate_otp() -> String {
    // Consider using a more secure OTP generation method if needed
    rand::random_range(100000..999999).to_string()
}


/// Checks if a client IP is allowed based on a list of allowed IP ranges [IPv4/IPv6] (CIDR notation)
/// Returns true if the client IP is within any of the allowed ranges, false otherwise
pub fn check_ip_range_allowed(client_ip: &str, allowed_ip_ranges: &[String]) -> bool {
    // If there is no allowed IP range specified, allow all IPs
    if allowed_ip_ranges.is_empty() {
        return true;
    }

    // Check if the client IP falls within any of the allowed IP ranges
    for range in allowed_ip_ranges {
        if check_ip_allowed(client_ip, range) {
            return true;
        }
    }

    false
}
