use crate::lexical_analysis::{Keyword, Token};

#[derive(Debug)]
pub(crate) enum Type {
    Void,
    Int,
}

impl Type {
    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        fn keyword_to_type(keyword: Keyword) -> anyhow::Result<Type> {
            match keyword {
                Keyword::Int => Ok(Type::Int),
                Keyword::Void => Ok(Type::Void),
                _ => anyhow::bail!("Expected type"),
            }
        }
        match tokens {
            [Token::Keyword(keyword), ..] => {
                Ok((&tokens[1..], keyword_to_type(keyword.to_owned())?))
            }
            [_, ..] => anyhow::bail!("Expected type"),
            [] => unreachable!(),
        }
    }
}
