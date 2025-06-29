use crate::position::InputReader;

/// skip all whitespace characters
pub(crate) fn skip_whitespace(reader: &mut InputReader) {
    while let Some(ch) = reader.peek() {
        if ch.is_whitespace() {
            reader.advance();
        } else {
            break;
        }
    }
}
