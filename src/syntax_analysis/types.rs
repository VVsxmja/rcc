use crate::lexical_analysis::{Keyword, Token};

#[derive(Debug)]
pub(crate) enum Type {
    Void,
    Int,
}

impl Type {
    pub(crate) fn parse<'a>(tokens: &'a [Token], target: &mut Option<Self>) -> anyhow::Result<&'a [Token]> {
        fn keyword_to_type(keyword: Keyword) -> Type {
            match keyword {
                Keyword::Int => Type::Int,
                Keyword::Void => Type::Void,
                _ => unreachable!(),
            }
        }
        match tokens {
            [Token::Keyword(kwtype @ (Keyword::Int | Keyword::Void)), ..] => {
                *target = Some(keyword_to_type(kwtype.to_owned()));
                Ok(&tokens[1..])
            }
            _ => {
                anyhow::bail!("Expected type");
            }
        }
    }
}