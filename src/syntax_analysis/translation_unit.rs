use crate::lexical_analysis::{Keyword, Token};

use super::declaration::Declaration;

#[derive(Debug)]
pub(crate) struct TranslationUnit(pub(crate) Vec<Declaration>);

impl TranslationUnit {
    pub(crate) fn parse(mut tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let mut body = Vec::new();
        loop {
            match tokens {
                [] => unreachable!(),
                [Token::End, tokens @ ..] => {
                    break Ok((tokens, TranslationUnit(body)));
                }
                [Token::Keyword(Keyword::Int | Keyword::Void), ..] => {
                    let (remain, decl) = Declaration::parse(tokens)?;
                    body.push(decl);
                    tokens = remain;
                }
                [_, ..] => anyhow::bail!("Expected declaration"),
            }
        }
    }
}
