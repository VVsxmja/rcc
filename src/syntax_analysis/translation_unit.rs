use crate::lexical_analysis::{Keyword, Token};

use super::top_level::TopLevel;

#[derive(Debug)]
pub(crate) struct TranslationUnit(Vec<TopLevel>);

impl TranslationUnit {
    pub(crate) fn parse<'a>(mut tokens: &'a [Token], target: &mut Vec<Self>) -> anyhow::Result<&'a [Token]> {
        let mut top_level = Vec::new();
        loop {
            tokens = match tokens {
                [] => unreachable!("Empty tokens"),
                [Token::End, ..] => {
                    target.push(TranslationUnit(top_level));
                    break Ok(&tokens[1..]);
                }
                [Token::Keyword(Keyword::Int | Keyword::Void), ..] => {
                    TopLevel::parse(tokens, &mut top_level)?
                }
                [_, ..] => anyhow::bail!("Expected top level declaration"),
            }
        }
    }
}