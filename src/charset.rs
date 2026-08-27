/// Character encoding/recode support
/// Per-channel/per-network charset detection and conversion

/// Map of common IRC charsets to encoding_rs names
pub fn charset_name(name: &str) -> &'static str {
    match name.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => "UTF-8",
        "ISO-8859-1" | "LATIN1" | "LATIN-1" | "ISO8859-1" => "ISO-8859-1",
        "ISO-8859-2" | "LATIN2" | "LATIN-2" | "ISO8859-2" => "ISO-8859-2",
        "ISO-8859-15" | "LATIN9" | "LATIN-9" | "ISO8859-15" => "ISO-8859-15",
        "WINDOWS-1252" | "CP1252" | "WIN1252" => "WINDOWS-1252",
        "WINDOWS-1250" | "CP1250" | "WIN1250" => "WINDOWS-1250",
        "WINDOWS-1251" | "CP1251" | "WIN1251" => "WINDOWS-1251",
        "ASCII" | "US-ASCII" => "ASCII",
        _ => "UTF-8",
    }
}

/// Try to convert bytes from one charset to UTF-8
/// Returns the converted string, or the original if conversion fails
pub fn recode_to_utf8(text: &[u8], from_charset: &str) -> String {
    if from_charset.is_empty() || from_charset.to_uppercase() == "UTF-8" {
        return String::from_utf8_lossy(text).to_string();
    }

    // Try encoding_rs for common charsets
    let encoding = encoding_rs::Encoding::for_label(from_charset.as_bytes());
    match encoding {
        Some(enc) => {
            let (decoded, _, had_errors) = enc.decode(text);
            if had_errors {
                // Fallback to UTF-8 lossy
                String::from_utf8_lossy(text).to_string()
            } else {
                decoded.into_owned()
            }
        }
        None => String::from_utf8_lossy(text).to_string(),
    }
}

/// Try to convert UTF-8 string to another charset
/// Returns the encoded bytes, or UTF-8 bytes if conversion fails
pub fn recode_from_utf8(text: &str, to_charset: &str) -> Vec<u8> {
    if to_charset.is_empty() || to_charset.to_uppercase() == "UTF-8" {
        return text.as_bytes().to_vec();
    }

    let encoding = encoding_rs::Encoding::for_label(to_charset.as_bytes());
    match encoding {
        Some(enc) => {
            let (encoded, _, _) = enc.encode(text);
            encoded.into_owned()
        }
        None => text.as_bytes().to_vec(),
    }
}

/// Detect charset from text content (heuristic)
pub fn detect_charset(text: &[u8]) -> &'static str {
    // Check for BOM
    if text.len() >= 3 && text[0] == 0xEF && text[1] == 0xBB && text[2] == 0xBF {
        return "UTF-8";
    }
    if text.len() >= 2 && text[0] == 0xFF && text[1] == 0xFE {
        return "UTF-16LE";
    }
    if text.len() >= 2 && text[0] == 0xFE && text[1] == 0xFF {
        return "UTF-16BE";
    }

    // Check if valid UTF-8
    if std::str::from_utf8(text).is_ok() {
        return "UTF-8";
    }

    // Check for common high-byte patterns
    let has_high_bytes = text.iter().any(|&b| b > 127);
    if has_high_bytes {
        // Default to ISO-8859-1 for non-UTF-8 high bytes
        "ISO-8859-1"
    } else {
        "ASCII"
    }
}

/// List of common charsets for tab completion
pub fn common_charsets() -> Vec<&'static str> {
    vec![
        "UTF-8", "ISO-8859-1", "ISO-8859-2", "ISO-8859-15",
        "WINDOWS-1252", "WINDOWS-1250", "WINDOWS-1251",
        "ASCII", "LATIN1", "LATIN2",
    ]
}
