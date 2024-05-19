mod token_matcher;
mod tokens;

use token_matcher::*;
pub(crate) use tokens::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn from_cursor(input: &str, cursor: usize) -> Self {
        let mut line: usize = 0;
        let mut column: usize = 0;
        for ch in input.chars().take(cursor) {
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Position { line, column }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

enum PositionOperation {
    NewLine,
    Next(usize),
}

fn checked_identifier(identifier: &str) -> anyhow::Result<String> {
    let identifier: Vec<_> = identifier.chars().collect();
    match &identifier[..] {
        [] => unreachable!("Zero-length identifier"),
        [ch, ..] if ch.is_numeric() => {
            anyhow::bail!("Identifier cannot start with numeric characters")
        }
        _ => Ok(identifier.into_iter().collect()),
    }
}

pub(crate) fn extract_tokens(input: &str) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut position = Position { column: 0, line: 0 };
    let mut it = input.chars().enumerate().peekable();

    while let Some((cursor, ch)) = it.next() {
        use PositionOperation::*;
        debug_assert_eq!(position, Position::from_cursor(&input, cursor));
        let position_operation = match ch {
            '\n' => NewLine,
            ch if ch.is_ascii_whitespace() => Next(1),
            _ => match TOKEN_MATCHER.get_token(&input[cursor..]) {
                Some((token, token_len)) => {
                    tracing::trace!("Token: {token:?}");
                    tokens.push(token);
                    for _ in 0..(token_len - 1) {
                        let _ = it.next();
                    }
                    Next(token_len)
                }
                None => {
                    // identifier or constant
                    let buffer: String = input[cursor..]
                        .chars()
                        .take_while(|ch| !ch.is_ascii_whitespace())
                        .collect();
                    let mut end: usize = 0;
                    for i in 0..(buffer.len()) {
                        if TOKEN_MATCHER.get_token(&buffer[i..]).is_some() {
                            break;
                        } else {
                            end = i + 1;
                        }
                    }
                    let buffer = &buffer[..end];
                    if let Ok(constant) = Constant::new(buffer) {
                        let token = Token::Constant(constant);
                        tracing::trace!("Constant: {token:?}");
                        tokens.push(token);
                    } else {
                        let token = Token::Identifier(checked_identifier(buffer)?);
                        tracing::trace!("Identifier: {token:?}");
                        tokens.push(token);
                    }
                    for _ in 0..(buffer.len() - 1) {
                        let _ = it.next();
                    }
                    Next(buffer.len())
                }
            },
        };
        match position_operation {
            NewLine => {
                position.column = 0;
                position.line += 1;
            }
            Next(len) => {
                position.column += len;
            }
        }
    }
    tokens.push(Token::End);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_from_cursor() {
        let input = "";
        let cursor = 0;
        let position = Position::from_cursor(input, cursor);
        assert_eq!(position.line, 0);
        assert_eq!(position.column, 0);
    }
}
