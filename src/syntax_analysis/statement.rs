use crate::{
    lexical_analysis::{Keyword, Symbol, Token},
    syntax_analysis::next,
};

use super::{block_statement::Block, expression::Expression};

#[derive(Debug)]
pub(crate) enum Statement {
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Block(Block),
    Expression(Expression),
    Jump(JumpStatement),
    Empty,
}

#[derive(Debug)]
pub(crate) enum JumpStatement {
    Return(Option<Expression>),
}

impl Statement {
    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        match tokens {
            [] => unreachable!(),
            [Token::Symbol(Symbol::Semicolon), remain_tokens @ ..] => {
                Ok((remain_tokens, Statement::Empty))
            }
            [Token::Keyword(Keyword::If), tokens @ ..] => {
                let (tokens, Token::Symbol(Symbol::LeftParen)) = next(tokens)? else {
                    anyhow::bail!("Expected \"(\"");
                };
                let (tokens, condition) = Expression::parse(tokens)?;
                let (tokens, Token::Symbol(Symbol::RightParen)) = next(tokens)? else {
                    anyhow::bail!("Expected \")\"");
                };
                let (tokens, true_branch) = Statement::parse(tokens)?;
                match tokens {
                    [Token::Keyword(Keyword::Else), tokens @ ..] => {
                        let (tokens, false_branch) = Statement::parse(tokens)?;
                        Ok((
                            tokens,
                            Statement::If(
                                condition,
                                Box::new(true_branch),
                                Some(Box::new(false_branch)),
                            ),
                        ))
                    }
                    _ => Ok((
                        tokens,
                        Statement::If(condition, Box::new(true_branch), None),
                    )),
                }
            }
            [Token::Keyword(Keyword::While), tokens @ ..] => {
                let (tokens, Token::Symbol(Symbol::LeftParen)) = next(tokens)? else {
                    anyhow::bail!("Expected \"(\"");
                };
                let (tokens, condition) = Expression::parse(tokens)?;
                let (tokens, Token::Symbol(Symbol::RightParen)) = next(tokens)? else {
                    anyhow::bail!("Expected \")\"");
                };
                let (tokens, body) = Statement::parse(tokens)?;
                Ok((tokens, Statement::While(condition, Box::new(body))))
            }
            [Token::Symbol(Symbol::LeftBrace), ..] => {
                let (tokens, block) = Block::parse(tokens)?;
                Ok((tokens, Statement::Block(block)))
            }
            [Token::Keyword(Keyword::Return), tokens @ ..] => match tokens {
                [Token::Symbol(Symbol::Semicolon), tokens @ ..] => {
                    Ok((tokens, Statement::Jump(JumpStatement::Return(None))))
                }
                _ => {
                    let (tokens, return_value) = Expression::parse(tokens)?;
                    let (tokens, Token::Symbol(Symbol::Semicolon)) = next(tokens)? else {
                        anyhow::bail!("Expected \";\"");
                    };
                    Ok((
                        tokens,
                        Statement::Jump(JumpStatement::Return(Some(return_value))),
                    ))
                }
            },
            _ => {
                let (tokens, expr) = Expression::parse(tokens)?;
                let (tokens, Token::Symbol(Symbol::Semicolon)) = next(tokens)? else {
                    anyhow::bail!("Expected \";\"");
                };
                Ok((tokens, Statement::Expression(expr)))
            }
        }
    }
}
