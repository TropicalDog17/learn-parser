use anyhow::Error;

use crate::scanner::{self, Scanner};

pub type Command = Vec<String>;

// $<length>\r\n<data>\r\n
fn parse_bulk_string(scanner: &mut Scanner) -> Result<String, Error> {
    let mut result = String::new();
    match scanner.pop() {
        Some(character) => {
            if !character.eq(&'$') {
                return Err(anyhow::format_err!(
                    "invalid character, expect $, get {}",
                    character
                ));
            }
            let length: usize;
            // Parse length
            match scanner.pop() {
                Some(l) => {
                    length = char::to_digit(*l, 10).unwrap() as usize;
                    if scanner.pop() != Some(&'\r') || scanner.pop() != Some(&'\n') {
                        return Err(anyhow::format_err!("Invalid character, expect CRLF"));
                    }
                    for _ in 0..length {
                        let c = scanner.pop().unwrap();
                        result.push(*c)
                    }

                    // check if encounter CRLF, if not return error
                    if scanner.pop() != Some(&'\r') || scanner.pop() != Some(&'\n') {
                        return Err(anyhow::format_err!("Invalid character, expect CRLF"));
                    }
                    return Ok(result);
                }
                None => {
                    return Err(anyhow::format_err!("Invalid length"));
                }
            }
        }
        None => return Err(anyhow::format_err!("invalid end of line ")),
    }
}

fn parse_array_command(scanner: &mut Scanner) -> Result<Command, Error> {
    let mut result = Command::new();
    match scanner.pop() {
        Some(character) => {
            if !character.eq(&'*') {
                return Err(anyhow::format_err!(
                    "invalid character, expect *, get {}",
                    character
                ));
            }
            let length: usize;
            match scanner.pop() {
                Some(l) => {
                    length = char::to_digit(*l, 10).unwrap() as usize;
                    if scanner.pop() != Some(&'\r') || scanner.pop() != Some(&'\n') {
                        return Err(anyhow::format_err!(
                            "Invalid character, expect CRLF after array length"
                        ));
                    }
                    for _ in 0..length {
                        let parsed_string = parse_bulk_string(scanner);
                        match parsed_string {
                            Ok(s) => result.push(s),
                            _ => {
                                return Err(anyhow::format_err!(
                                    "Invalid element, expect bulk string"
                                ))
                            }
                        }
                    }

                    // After consume all string, should be the end
                    if scanner.pop().is_some() {
                        return Err(anyhow::format_err!(
                            "invalid character, should be end of array"
                        ));
                    }
                    Ok(result)
                }
                None => return Err(anyhow::format_err!("invalid end of line ")),
            }
        }
        None => return Err(anyhow::format_err!("invalid end of line ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("$5\r\nhello\r\n", "hello", 11)]
    #[case("$8\r\nsunlight\r\n", "sunlight", 14)]
    #[case("$3\r\nhaha\r\n", "", 0)]
    #[case("$1\r\nx\r\n", "x", 7)]

    fn test_parse_bulk_string(
        #[case] test_str: &str,
        #[case] expected: &str,
        #[case] expected_cursor: usize,
    ) {
        let mut scanner = scanner::Scanner::new(test_str);
        let actual: Result<String, Error> = parse_bulk_string(&mut scanner);
        if expected == "" {
            assert!(actual.is_err())
        } else {
            assert!(actual.is_ok());
            assert_eq!(expected, actual.unwrap());
            assert_eq!(expected_cursor, scanner.cursor())
        }
    }

    #[rstest]
    #[case("*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n", vec!["LLEN", "mylist"])]
    #[case("*2\r\n$4\r\nLLEN\r\n$5\r\nmylist\r\n", vec!["-1"])]

    fn test_parse_array_command(#[case] test_str: &str, #[case] expected: Vec<&str>) {
        let mut scanner = scanner::Scanner::new(test_str);
        let actual: Result<Command, Error> = parse_array_command(&mut scanner);

        if expected.starts_with(&vec!["-1"]) {
            assert!(actual.is_err())
        } else {
            // dbg!(actual);
            // assert!(actual.is_ok());
            assert_eq!(expected, actual.unwrap());
        }
    }
}
