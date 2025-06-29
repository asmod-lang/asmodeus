use crate::error::LexerError;
use crate::position::InputReader;

/// decimal, hexadecimal or binary numbers
pub(crate) fn read_number(reader: &mut InputReader) -> Result<String, LexerError> {
    let mut number = String::new();
    let start_line = reader.line();
    let start_column = reader.column();

    // negative numbers
    if reader.peek() == Some('-') {
        number.push('-');
        reader.advance();
    }

    // hex (0x) or binary (0b) prefix
    if reader.peek() == Some('0') {
        if let Some(next) = reader.peek_ahead(1) {
            if next == 'x' || next == 'X' {
                parse_hexadecimal(reader, &mut number, start_line, start_column)
            } else if next == 'b' || next == 'B' {
                parse_binary(reader, &mut number, start_line, start_column)
            } else {
                parse_decimal_starting_with_zero(reader, &mut number)
            }
        } else {
            // just '0'
            number.push('0');
            reader.advance();
            Ok(number)
        }
    } else {
        parse_decimal(reader, &mut number);
        validate_number(number, start_line, start_column)
    }
}

fn parse_hexadecimal(
    reader: &mut InputReader, 
    number: &mut String, 
    start_line: usize, 
    start_column: usize
) -> Result<String, LexerError> {
    let next = reader.peek_ahead(1).unwrap();
    
    // preserve original case
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
            value: number.clone(),
        });
    }
    
    Ok(number.clone())
}

fn parse_binary(
    reader: &mut InputReader, 
    number: &mut String, 
    start_line: usize, 
    start_column: usize
) -> Result<String, LexerError> {
    let next = reader.peek_ahead(1).unwrap();
    
    // preserve original case
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
            value: number.clone(),
        });
    }
    
    Ok(number.clone())
}

fn parse_decimal_starting_with_zero(reader: &mut InputReader, number: &mut String) -> Result<String, LexerError> {
    // regular decimal starting with 0
    while let Some(ch) = reader.peek() {
        if ch.is_ascii_digit() {
            number.push(ch);
            reader.advance();
        } else {
            break;
        }
    }
    Ok(number.clone())
}

fn parse_decimal(reader: &mut InputReader, number: &mut String) {
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

fn validate_number(number: String, start_line: usize, start_column: usize) -> Result<String, LexerError> {
    if number.is_empty() || number == "-" {
        return Err(LexerError::InvalidNumberFormat {
            line: start_line,
            column: start_column,
            value: number,
        });
    }
    Ok(number)
}
