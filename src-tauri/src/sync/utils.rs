use num_traits::ToPrimitive;

/// Convert bytes to human readable format
///
/// ## Arguments
///
/// * `bytes` - bytes to convert
///
/// ## Returns
///
/// The bytes in human readable format (e.g. 1.5 MiB)
pub fn human_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    let mut bytes = bytes.to_f64().unwrap_or(0.0);
    let mut index = 0;

    while bytes >= 1024.0 && index < UNITS.len() - 1 {
        bytes /= 1024.0;
        index += 1;
    }

    let n: usize = if bytes >= 100.0 {
        0
    } else if bytes >= 1.0 {
        1
    } else if bytes > 0.0 {
        2
    } else {
        0
    };
    format!("{bytes:.n$} {}", UNITS[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(1024), "1.0 KiB");
        assert_eq!(human_bytes(1024 * 1024), "1.0 MiB");
        assert_eq!(human_bytes(123 * 1024), "123 KiB");
    }
}
