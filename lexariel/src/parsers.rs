//! specialized parsers for different token types

use crate::error::LexerError;
use crate::position::InputReader;

/// alphanumeric + underscore
pub(crate) fn read_identifier(reader: &mut InputReader) -> String {
    let mut identifier = String::new();

    while let Some(ch) = reader.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            identifier.push(ch);
            reader.advance();
        } else {
            break;
        }
    }

    identifier
}

/// decimal, hexadecimal or binary
pub(crate) fn read_number(reader: &mut InputReader) -> Result<String, LexerError> {
    let mut number = String::new();
    let start_line = reader.line();
    let start_column = reader.column();

    // negative numbers
    if reader.peek() == Some('-') {
        number.push('-');
        reader.advance();
    }

    // check for hex (0x) or binary (0b) prefix
    if reader.peek() == Some('0') {
        if let Some(next) = reader.peek_ahead(1) {
            if next == 'x' || next == 'X' {
                // hex - preserve original case
                number.push('0');
                reader.advance(); // skip '0'
                number.push(next); // preserve original 'x' or 'X'
                reader.advance(); // skip 'x'/'X'
                
                let mut has_digits = false;
                while let Some(ch) = reader.peek() {
                    if ch.is_ascii_hexdigit() {
                        number.push(ch);
                        reader.advance();
                        has_digits = true;
                    } else {
                        break;
                    }
                }
                
                if !has_digits {
                    return Err(LexerError::InvalidNumberFormat {
                        line: start_line,
                        column: start_column,
                        value: number,
                    });
                }
            } else if next == 'b' || next == 'B' {
                // binary - preserve original case
                number.push('0');
                reader.advance(); // skip '0'
                number.push(next); // preserve original 'b' or 'B'
                reader.advance(); // skip 'b'/'B'
                
                let mut has_digits = false;
                while let Some(ch) = reader.peek() {
                    if ch == '0' || ch == '1' {
                        number.push(ch);
                        reader.advance();
                        has_digits = true;
                    } else {
                        break;
                    }
                }
                
                if !has_digits {
                    return Err(LexerError::InvalidNumberFormat {
                        line: start_line,
                        column: start_column,
                        value: number,
                    });
                }
            } else {
                // regular decimal starting with 0
                while let Some(ch) = reader.peek() {
                    if ch.is_ascii_digit() {
                        number.push(ch);
                        reader.advance();
                    } else {
                        break;
                    }
                }
            }
        } else {
            // just '0'
            number.push('0');
            reader.advance();
        }
    } else {
        // regular decimal number
        while let Some(ch) = reader.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                reader.advance();
            } else {
                break;
            }
        }
    }

    if number.is_empty() || number == "-" {
        return Err(LexerError::InvalidNumberFormat {
            line: start_line,
            column: start_column,
            value: number,
        });
    }

    Ok(number)
}

pub(crate) fn skip_whitespace(reader: &mut InputReader) {
    while let Some(ch) = reader.peek() {
        if ch.is_whitespace() {
            reader.advance();
        } else {
            break;
        }
    }
}

/// starting with ; or //
pub(crate) fn skip_comment(reader: &mut InputReader) {
    if let Some(ch) = reader.peek() {
        if ch == ';' {
            // skip until end of line
            while let Some(ch) = reader.advance() {
                if ch == '\n' {
                    break;
                }
            }
        } else if ch == '/' && reader.peek_ahead(1) == Some('/') {
            // skip // comment
            reader.advance(); // skip first /
            reader.advance(); // skip second /
            while let Some(ch) = reader.advance() {
                if ch == '\n' {
                    break;
                }
            }
        }
    }
}
