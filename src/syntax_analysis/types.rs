use crate::lexical_analysis::{Keyword, Token};

#[derive(Debug)]
pub enum Type {
    Void,
    Int,
}

impl Type {
    pub fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        match tokens {
            [Token::Keyword(keyword), ..] => {
                let kwtype = match keyword {
                    Keyword::Int => Type::Int,
                    Keyword::Void => Type::Void,
                    _ => anyhow::bail!("Expected type"),
                };
                Ok((&tokens[1..], kwtype))
            }
            [_, ..] => anyhow::bail!("Expected type"),
            [] => unreachable!(),
        }
    }
}
