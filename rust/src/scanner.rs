use std::ops::Add;
use std::process::{ExitCode, Termination};
pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }
    /// Returns the current cursor. Useful for reporting errors.

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }
    /// Returns true if further progress is not possible.
    ///
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns the next character (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            None => None,
            Some(character) => {
                self.cursor += 1;
                Some(character)
            }
        }
    }
}
impl Termination for Scanner {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    pub fn scanner() -> Scanner {
        Scanner::new("lorem ipsum")
    }

    #[rstest]
    #[case(0, Some('l'))]
    #[case(5, Some(' '))]
    #[case(10, Some('m'))]
    #[case(11, None)]
    pub fn test_peek(mut scanner: Scanner, #[case] cursor: usize, #[case] expected: Option<char>) {
        scanner.cursor = cursor;
        assert_eq!(scanner.peek(), expected.as_ref())
    }
}
