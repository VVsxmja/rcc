use crate::{
    lexical_analysis::{Keyword, Symbol, Token},
    syntax_analysis::expression::eval,
};

use super::{
    block_statement::Block, expression::Expression, next,
    parameter_definition::ParameterDefinition, types::Type,
};

#[derive(Debug)]
pub enum Declaration {
    Variable(Type, String, Option<Expression>),
    Function(Type, String, Vec<ParameterDefinition>, Option<Block>),
}

impl Declaration {
    pub fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, decl_type) = Type::parse(tokens)?;
        let (tokens, Token::Identifier(id)) = next(tokens)? else {
            anyhow::bail!("Expected identifier");
        };
        let (mut tokens, token_after_id) = next(tokens)?;
        match token_after_id {
            Token::Symbol(Symbol::LeftParen) => {
                let mut params = Vec::new();
                match tokens {
                    [Token::Keyword(Keyword::Void), Token::Symbol(Symbol::RightParen), remain @ ..]
                    | [Token::Symbol(Symbol::RightParen), remain @ ..] => tokens = remain,
                    _ => loop {
                        let (remain, param) = ParameterDefinition::parse(tokens)?;
                        params.push(param);
                        let (remain, token_after_param) = next(remain)?;
                        tokens = remain;
                        match token_after_param {
                            Token::Symbol(Symbol::Comma) => continue,
                            Token::Symbol(Symbol::RightParen) => break,
                            _ => anyhow::bail!("Expected \",\" or \";\""),
                        }
                    },
                }
                if let [Token::Symbol(Symbol::Semicolon), tokens @ ..] = tokens {
                    let func_decl = Declaration::Function(decl_type, id, params, None);
                    tracing::trace!("Function definition: {func_decl:?}");
                    Ok((tokens, func_decl))
                } else {
                    let (tokens, block_stmt) = Block::parse(tokens)?;
                    let func_decl = Declaration::Function(decl_type, id, params, Some(block_stmt));
                    tracing::trace!("Function definition: {func_decl:?}");
                    Ok((tokens, func_decl))
                }
            }
            Token::Symbol(Symbol::Semicolon) => {
                let var_decl = Declaration::Variable(decl_type, id, None);
                tracing::trace!("Variable declaration: {var_decl:?}");
                Ok((tokens, var_decl))
            }
            Token::Symbol(Symbol::Equal) => {
                let (tokens, expr) = Expression::parse(tokens)?;
                let var_decl = Declaration::Variable(decl_type, id, Some(eval(expr)));
                tracing::trace!("Variable declaration: {var_decl:?}");
                let (tokens, Token::Symbol(Symbol::Semicolon)) = next(tokens)? else {
                    anyhow::bail!("Expected \";\"");
                };
                Ok((tokens, var_decl))
            }
            _ => anyhow::bail!("Expected function or variable"),
        }
    }
}
