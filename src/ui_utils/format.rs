// utils/format.rs
// =========================================
// Formatting utilities for NEAR Explorer
// =========================================
// =========================================

/// Base58 alphabet
const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Encode bytes to base58
pub fn encode_base58(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let mut digits = vec![0u8; data.len() * 2];
    let mut length = 1usize;

    for &byte in data {
        let mut carry = byte as usize;
        for digit in digits.iter_mut().take(length) {
            carry += *digit as usize * 256;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits[length] = (carry % 58) as u8;
            length += 1;
            carry /= 58;
        }
    }

    let mut result = String::with_capacity(length);
    for _ in 0..data.iter().take_while(|&&b| b == 0).count() {
        result.push(BASE58_ALPHABET[0] as char);
    }
    for &digit in digits[..length].iter().rev() {
        result.push(BASE58_ALPHABET[digit as usize] as char);
    }

    result
}

/// Decode base58 to bytes
pub fn decode_base58(encoded: &str) -> Option<Vec<u8>> {
    if encoded.is_empty() {
        return Some(Vec::new());
    }

    let mut bytes = vec![0u8; encoded.len()];
    let mut length = 1usize;

    for char in encoded.chars() {
        let val = BASE58_ALPHABET.iter().position(|&b| b == char as u8)? as usize;
        let mut carry = val;
        for byte in bytes.iter_mut().take(length) {
            carry += *byte as usize * 58;
            *byte = (carry % 256) as u8;
            carry /= 256;
        }
        while carry > 0 {
            bytes[length] = (carry % 256) as u8;
            length += 1;
            carry /= 256;
        }
    }

    let mut result = Vec::new();
    for _ in 0..encoded
        .chars()
        .take_while(|&c| c == BASE58_ALPHABET[0] as char)
        .count()
    {
        result.push(0);
    }
    result.extend(bytes[..length].iter().rev().copied());

    Some(result)
}

/// Format a large number with commas (for display)
pub fn format_number_with_commas(num: u64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Format yoctoNEAR to human-readable NEAR amount
pub fn format_near_amount(yocto_near: &str) -> String {
    let len = yocto_near.len();
    if len <= 24 {
        // Less than 1 NEAR
        let padded = format!("{:0>24}", yocto_near);
        let int_part = &padded[..24 - 24];
        let frac_part = &padded[24 - 24..];
        let frac_trimmed = frac_part.trim_end_matches('0');
        if int_part.is_empty() || int_part.parse::<u64>().unwrap_or(0) == 0 {
            if frac_trimmed.is_empty() {
                return "0".to_string();
            }
            return format!("0.{}", frac_trimmed);
        }
        if frac_trimmed.is_empty() {
            return int_part.to_string();
        }
        format!("{}.{}", int_part, frac_trimmed)
    } else {
        // More than 1 NEAR
        let int_part = &yocto_near[..len - 24];
        let frac_part = &yocto_near[len - 24..];
        let frac_trimmed = frac_part.trim_end_matches('0');
        if frac_trimmed.is_empty() {
            return int_part.to_string();
        }
        format!("{}.{}", int_part, frac_trimmed)
    }
}

/// Format gas amount (convert to Tgas for large values)
pub fn format_gas_amount(gas: u64) -> String {
    if gas >= 1_000_000_000_000u64 {
        format!("{:.2} Tgas", gas as f64 / 1_000_000_000_000.0)
    } else if gas >= 1_000_000_000u64 {
        format!("{:.2} Ggas", gas as f64 / 1_000_000_000.0)
    } else {
        format!("{} gas", gas)
    }
}

/// Truncate a string in the middle with ellipsis
pub fn truncate_middle(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    if max_len < 6 {
        return s[..max_len.min(s.len())].to_string();
    }
    let start_len = (max_len - 3) / 2;
    let end_len = (max_len - 3) - start_len;
    format!("{}...{}", &s[..start_len], &s[s.len() - end_len..])
}

/// Parse timestamp from nanoseconds string to chrono DateTime
pub fn parse_timestamp_ns(ts: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let nanos: i64 = ts.parse().ok()?;
    let secs = nanos / 1_000_000_000;
    let nsecs = (nanos % 1_000_000_000) as u32;
    chrono::DateTime::from_timestamp(secs, nsecs)
}

/// Format timestamp as "time ago" string
pub fn format_time_ago(ts: &str) -> String {
    if let Some(dt) = parse_timestamp_ns(ts) {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(dt);

        let secs = duration.num_seconds();
        if secs < 60 {
            return format!("{}s ago", secs);
        }
        let mins = secs / 60;
        if mins < 60 {
            return format!("{}m ago", mins);
        }
        let hours = mins / 60;
        if hours < 24 {
            return format!("{}h ago", hours);
        }
        let days = hours / 24;
        if days < 30 {
            return format!("{}d ago", days);
        }
        let months = days / 30;
        if months < 12 {
            return format!("{}mo ago", months);
        }
        let years = months / 12;
        format!("{}y ago", years)
    } else {
        "Unknown".to_string()
    }
}
// =========================================
// copyright 2026 by sleet.near
