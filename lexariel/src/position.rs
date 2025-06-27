//! position tracking and character navigation

/// helper struct for tracking position in source code
#[derive(Debug, Clone)]
pub(crate) struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    pub fn advance_char(&mut self, ch: char) {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

/// helper struct for managing input stream and position
pub(crate) struct InputReader {
    input: Vec<char>,
    position: usize,
    location: Position,
}

impl InputReader {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            location: Position::new(),
        }
    }

    /// returns the current character without advancing
    pub fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// returns the character at the given offset without advancing
    pub fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    /// advances to the next character and returns it
    pub fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input.get(self.position) {
            self.position += 1;
            self.location.advance_char(*ch);
            Some(*ch)
        } else {
            None
        }
    }

    /// current line number
    pub fn line(&self) -> usize {
        self.location.line
    }

    /// current column number
    pub fn column(&self) -> usize {
        self.location.column
    }
}
