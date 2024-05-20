use crate::lexical_analysis::{Keyword, Symbol, Token};

use super::{declaration::Declaration, next, statement::Statement};

#[derive(Debug)]
pub(crate) struct Block(pub(crate) Vec<BlockInner>);

#[derive(Debug)]
pub(crate) enum BlockInner {
    Declaration(Declaration),
    Statement(Statement),
}

impl Block {
    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (mut tokens, Token::Symbol(Symbol::LeftBrace)) = next(tokens)? else {
            anyhow::bail!("Expected block statement. Block statement should start with \"{{\"");
        };
        let mut body = Vec::new();
        loop {
            match tokens {
                [Token::Symbol(Symbol::RightBrace), tokens @ ..] => {
                    break Ok((tokens, Block(body)));
                }
                [Token::Keyword(Keyword::Int | Keyword::Void), ..] => {
                    let (remain, decl) = Declaration::parse(tokens)?;
                    tokens = remain;
                    body.push(BlockInner::Declaration(decl));
                }
                _ => {
                    let (remain, stmts) = Statement::parse(tokens)?;
                    tokens = remain;
                    body.push(BlockInner::Statement(stmts));
                }
            }
        }
    }
}
