use crate::lexical_analysis::{Keyword, Token};

use super::{block_statement::BlockInner, declaration::Declaration};

#[derive(Debug)]
pub(crate) struct TranslationUnit(Vec<BlockInner>);

impl TranslationUnit {
    pub(crate) fn parse<'a>(
        mut tokens: &'a [Token],
        target: &mut Option<Self>,
    ) -> anyhow::Result<&'a [Token]> {
        let mut body = Vec::new();
        loop {
            match tokens {
                [] => unreachable!(),
                [Token::End, ..] => {
                    *target = Some(TranslationUnit(body));
                    break Ok(tokens);
                }
                [Token::Keyword(Keyword::Int | Keyword::Void), ..] => {
                    let mut decl = None;
                    tokens = Declaration::parse(tokens, &mut decl)?;
                    let Some(decl) = decl else { unreachable!() };
                    body.push(BlockInner::Declaration(decl));
                }
                [_, ..] => anyhow::bail!("Expected top level declaration"),
            }
        }
    }
}
