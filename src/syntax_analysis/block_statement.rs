use crate::lexical_analysis::{Symbol, Token};

use super::{next, statement::Statement};

#[derive(Debug)]
pub(crate) struct BlockStatement(Vec<Statement>);

impl BlockStatement {
    pub(crate) fn parse<'a>(tokens: &'a [Token], target: &mut Option<Self>) -> anyhow::Result<&'a [Token]> {
        let (Token::Symbol(Symbol::LeftBrace), mut tokens) = next(tokens)? else {
            anyhow::bail!("Expected block statement. Block statement should start with '{{'");
        };
        let mut stmts = Vec::new();
        loop {
            match tokens {
                [Token::Symbol(Symbol::RightBrace), ..] => {
                    *target = Some(BlockStatement(stmts));
                    break Ok(tokens);
                }
                _ => {
                    tokens = Statement::parse(tokens, &mut stmts)?;
                }
            }
        }
    }
}