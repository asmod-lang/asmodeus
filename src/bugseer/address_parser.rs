use crate::error::AsmodeusError;

pub fn parse_address(addr_str: &str) -> Result<u16, AsmodeusError> {
    if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        // hex
        u16::from_str_radix(&addr_str[2..], 16).map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid hexadecimal address: {}", addr_str))
        })
    } else {
        // dec
        addr_str.parse::<u16>().map_err(|_| {
            AsmodeusError::UsageError(format!("Invalid decimal address: {}", addr_str))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal_address() {
        assert_eq!(parse_address("123").unwrap(), 123);
        assert_eq!(parse_address("0").unwrap(), 0);
        assert_eq!(parse_address("2047").unwrap(), 2047);
    }

    #[test]
    fn test_parse_hex_address() {
        assert_eq!(parse_address("0x7B").unwrap(), 123);
        assert_eq!(parse_address("0X7B").unwrap(), 123);
        assert_eq!(parse_address("0x0").unwrap(), 0);
        assert_eq!(parse_address("0x7FF").unwrap(), 2047);
    }

    #[test]
    fn test_invalid_addresses() {
        assert!(parse_address("abc").is_err());
        assert!(parse_address("0xGHI").is_err());
        assert!(parse_address("").is_err());
    }
}
