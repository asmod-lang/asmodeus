use crate::position::InputReader;

/// comments starting with ; or //
pub(crate) fn skip_comment(reader: &mut InputReader) {
    if let Some(ch) = reader.peek() {
        if ch == ';' {
            skip_semicolon_comment(reader);
        } else if ch == '/' && reader.peek_ahead(1) == Some('/') {
            skip_double_slash_comment(reader);
        }
    }
}

fn skip_semicolon_comment(reader: &mut InputReader) {
    while let Some(ch) = reader.advance() {
        if ch == '\n' {
            break;
        }
    }
}

fn skip_double_slash_comment(reader: &mut InputReader) {
    // "//" comment
    reader.advance(); // first /
    reader.advance(); // second /
    while let Some(ch) = reader.advance() {
        if ch == '\n' {
            break;
        }
    }
}
