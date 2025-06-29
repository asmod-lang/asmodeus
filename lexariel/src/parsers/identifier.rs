use crate::position::InputReader;

/// alphanumeric identifiers with underscores
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
