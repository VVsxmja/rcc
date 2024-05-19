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
    Empty,
}

impl Statement {
    #[allow(unused_variables)]
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Option<Statement>,
    ) -> anyhow::Result<&'a [Token]> {
        match tokens {
            [] => unreachable!(),
            [Token::Symbol(Symbol::Semicolon), remain_tokens @ ..] => {
                *target = Some(Statement::Empty);
                Ok(remain_tokens)
            }
            [Token::Keyword(Keyword::If), tokens @ ..] => {
                let (Token::Symbol(Symbol::LeftParen), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected left parameter (\"(\")");
                };
                let mut expr = None;
                let tokens = Expression::parse(tokens, &mut expr)?;
                let Some(condition) = expr else {
                    unreachable!();
                };
                let (Token::Symbol(Symbol::RightParen), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected right parameter (\")\")");
                };
                let mut true_branch = None;
                let tokens = Statement::parse(tokens, &mut true_branch)?;
                let Some(true_branch) = true_branch else {
                    unreachable!();
                };
                match tokens {
                    [Token::Keyword(Keyword::Else), tokens @ ..] => {
                        let mut false_branch = None;
                        let tokens = Statement::parse(tokens, &mut false_branch)?;
                        let Some(false_branch) = false_branch else {
                            unreachable!();
                        };
                        *target = Some(Statement::If(
                            condition,
                            Box::new(true_branch),
                            Some(Box::new(false_branch)),
                        ));
                        Ok(tokens)
                    }
                    _ => {
                        *target = Some(Statement::If(condition, Box::new(true_branch), None));
                        Ok(tokens)
                    }
                }
            }
            [Token::Keyword(Keyword::While), tokens @ ..] => {
                let (Token::Symbol(Symbol::LeftParen), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected left parameter (\"(\")");
                };
                let mut expr = None;
                let tokens = Expression::parse(tokens, &mut expr)?;
                let Some(condition) = expr else {
                    unreachable!();
                };
                let (Token::Symbol(Symbol::RightParen), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected right parameter (\")\")");
                };
                let mut body = None;
                let tokens = Statement::parse(tokens, &mut body)?;
                let Some(body) = body else {
                    unreachable!();
                };
                match tokens {
                    [Token::Keyword(Keyword::Else), tokens @ ..] => {
                        let mut false_branch = None;
                        let tokens = Statement::parse(tokens, &mut false_branch)?;
                        let Some(false_branch) = false_branch else {
                            unreachable!();
                        };
                        *target = Some(Statement::If(
                            condition,
                            Box::new(body),
                            Some(Box::new(false_branch)),
                        ));
                        Ok(tokens)
                    }
                    _ => {
                        *target = Some(Statement::If(condition, Box::new(body), None));
                        Ok(tokens)
                    }
                }
            }
            _ => {
                let mut expr = None;
                let tokens = Expression::parse(tokens, &mut expr)?;
                let Some(expr) = expr else {
                    unreachable!();
                };
                *target = Some(Statement::Expression(expr));
                Ok(tokens)
            }
        }
    }
}
