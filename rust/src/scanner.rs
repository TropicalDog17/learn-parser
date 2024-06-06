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

    /// Advance the cursor if match exact, return true, else return false
    pub fn scan(&mut self, pat: &str) -> bool {
        match self
            .characters
            .get(self.cursor + 1..self.cursor + pat.len() + 1)
        {
            Some(substring) => {
                if substring.iter().collect::<String>() == pat.to_string() {
                    for _ in 0..pat.len() + 1 {
                        self.pop();
                    }
                    return true;
                }
                return false;
            }
            None => return false,
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

    #[fixture]
    pub fn redis_scanner() -> Scanner {
        Scanner::new("*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n")
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
    #[rstest]
    #[case(0, "orem", true, 5)]
    #[case(0, "oram", false, 0)]
    #[case(5, "ipsum", true, 11)]
    pub fn test_scan(
        mut scanner: Scanner,
        #[case] cursor: usize,
        #[case] pat: &str,
        #[case] expected: bool,
        #[case] expected_cursor: usize,
    ) {
        scanner.cursor = cursor;
        assert_eq!(expected, scanner.scan(pat));
        assert_eq!(expected_cursor, scanner.cursor);
    }

    #[rstest]
    #[case(5, "\r\n", true, 8)]

    pub fn test_scan_2(
        mut redis_scanner: Scanner,
        #[case] cursor: usize,
        #[case] pat: &str,
        #[case] expected: bool,
        #[case] expected_cursor: usize,
    ) {
        redis_scanner.cursor = cursor;
        assert_eq!(expected, redis_scanner.scan(pat));
        assert_eq!(expected_cursor, redis_scanner.cursor);
    }
}
