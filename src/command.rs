#[derive(Debug, PartialEq, Eq)]

pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Exit,
}

#[derive(Debug, PartialEq, Eq)]

pub enum ParserError {
    EmptyCommand,
    InvalidCommand(String),
    InvalidArgument(String),
}

pub fn parse(input: &str) -> Result<Command, ParserError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParserError::EmptyCommand);
    }

    let mut parts = input.split_whitespace();
    let operation = parts.next().unwrap().to_ascii_uppercase();
    match operation.as_str() {
        "SET" => {
            let key = parts.next().ok_or_else(|| {
                ParserError::InvalidArgument("Usage: Set <key> <value>".to_string())
            })?;

            let value = parts.collect::<Vec<_>>().join(" ");

            if value.is_empty() {
                return Err(ParserError::InvalidArgument(
                    "Usage:Set <key> <value>".to_string(),
                ));
            }

            Ok(Command::Set {
                key: key.to_string(),
                value,
            })
        }

        "GET" => {
            let key = parts
                .next()
                .ok_or_else(|| ParserError::InvalidArgument("usage: Get <key>".to_string()))?;

            if parts.next().is_some() {
                return Err(ParserError::InvalidArgument("usage: Get <key>".to_string()));
            }

            Ok(Command::Get {
                key: key.to_string(),
            })
        }

        "DEL" => {
            let key = parts
                .next()
                .ok_or_else(|| ParserError::InvalidArgument("Usage: DEL <key>".to_string()))?;

            if parts.next().is_some() {
                return Err(ParserError::InvalidArgument("Usage: DEL <key>".to_string()));
            }

            Ok(Command::Del {
                key: key.to_string(),
            })
        }

        "EXIT" | "QUIT" => {
            if parts.next().is_some() {
                return Err(ParserError::InvalidArgument("usage: Exit".to_string()));
            }
            Ok(Command::Exit)
        }

        _ => Err(ParserError::InvalidCommand(operation)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn parses_set_with_spaces_in_value() {
        assert_eq!(
            parse("SET greeting hello world"),
            Ok(Command::Set {
                key: "greeting".to_string(),
                value: "hello world".to_string(),
            })
        );
    }

    #[test]
    fn commands_are_case_insensitive() {
        assert_eq!(
            parse("get name"),
            Ok(Command::Get {
                key: "name".to_string(),
            })
        );
    }

    #[test]
    fn rejects_set_without_value() {
        assert_eq!(
            parse("SET name"),
            Err(ParserError::InvalidArgument(
                "Usage: SET <key> <value>".to_string(),
            ))
        );
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse("   "), Err(ParserError::EmptyCommand));
    }
}
