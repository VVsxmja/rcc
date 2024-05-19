use crate::lexical_analysis::{Keyword, Symbol, Token};

use super::{declaration::Declaration, next, statement::Statement};

#[derive(Debug)]
pub(crate) struct Block(Vec<BlockInner>);

#[derive(Debug)]
pub(crate) enum BlockInner {
    Declaration(Declaration),
    Statement(Statement),
}

impl Block {
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Option<Self>,
    ) -> anyhow::Result<&'a [Token]> {
        let (Token::Symbol(Symbol::LeftBrace), mut tokens) = next(tokens)? else {
            anyhow::bail!("Expected block statement. Block statement should start with '{{'");
        };
        let mut body = Vec::new();
        loop {
            match tokens {
                [Token::Symbol(Symbol::RightBrace), ..] => {
                    *target = Some(Block(body));
                    break Ok(tokens);
                }
                [Token::Keyword(Keyword::Int | Keyword::Void), ..] => {
                    let mut decl = None;
                    tokens = Declaration::parse(tokens, &mut decl)?;
                    let Some(decl) = decl else { unreachable!() };
                    body.push(BlockInner::Declaration(decl));
                }
                _ => {
                    let mut stmts = None;
                    tokens = Statement::parse(tokens, &mut stmts)?;
                    let Some(stmts) = stmts else { unreachable!() };
                    body.push(BlockInner::Statement(stmts));
                }
            }
        }
    }
}
