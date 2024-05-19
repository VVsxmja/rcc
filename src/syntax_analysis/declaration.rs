use crate::lexical_analysis::{Keyword, Symbol, Token};

use super::{
    block_statement::Block, expression::Expression, next,
    parameter_definition::ParameterDefinition, types::Type,
};

#[derive(Debug)]
pub(crate) enum Declaration {
    Variable(Type, String, Option<Expression>),
    Function(Type, String, Vec<ParameterDefinition>, Option<Block>),
}

impl Declaration {
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Option<Self>,
    ) -> anyhow::Result<&'a [Token]> {
        let mut decl_type = None;
        let tokens = Type::parse(tokens, &mut decl_type)?;
        let Some(decl_type) = decl_type else {
            anyhow::bail!("Expected type");
        };
        tracing::trace!("Top level type: {decl_type:?}");
        let (Token::Identifier(id), tokens) = next(tokens)? else {
            anyhow::bail!("Expected identifier");
        };
        tracing::trace!("Top level id: {id:?}");
        let (token_after_id, mut tokens) = next(tokens)?;
        let tokens = match token_after_id {
            Token::Symbol(Symbol::LeftParen) => {
                let mut params = Vec::new();
                match tokens {
                    [Token::Keyword(Keyword::Void), Token::Symbol(Symbol::RightParen), ..]
                    | [Token::Symbol(Symbol::RightParen), ..] => (),
                    _ => loop {
                        tokens = ParameterDefinition::parse(tokens, &mut params)?;
                        let (token_after_param, remain_tokens) = next(tokens)?;
                        tokens = remain_tokens;
                        match token_after_param {
                            Token::Symbol(Symbol::Comma) => continue,
                            Token::Symbol(Symbol::RightParen) => break,
                            _ => anyhow::bail!("Expected comma or semicolon"),
                        }
                    },
                }
                match tokens {
                    [Token::Symbol(Symbol::Semicolon), remain_tokens @ ..] => {
                        let func_decl = Declaration::Function(decl_type, id, params, None);
                        tracing::trace!("Function definition: {func_decl:?}");
                        *target = Some(func_decl);
                        remain_tokens
                    }
                    _ => {
                        let mut block_stmt = None;
                        tokens = Block::parse(tokens, &mut block_stmt)?;
                        let Some(block_stmt) = block_stmt else {
                            unreachable!();
                        };
                        let func_decl =
                            Declaration::Function(decl_type, id, params, Some(block_stmt));
                        tracing::trace!("Function definition: {func_decl:?}");
                        *target = Some(func_decl);
                        tokens
                    }
                }
            }
            Token::Symbol(Symbol::Semicolon) => {
                let var_decl = Declaration::Variable(decl_type, id, None);
                tracing::trace!("Variable declaration: {var_decl:?}");
                *target = Some(var_decl);
                tokens
            }
            Token::Symbol(Symbol::Assign) => {
                let mut expr = None;
                let tokens = Expression::parse(tokens, &mut expr)?;
                let Some(expr) = expr else {
                    unreachable!();
                };
                let variable_declaration = Declaration::Variable(decl_type, id, Some(expr));
                tracing::trace!("Variable declaration: {variable_declaration:?}");
                *target = Some(variable_declaration);
                let (Token::Symbol(Symbol::Semicolon), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected semicolon");
                };
                tokens
            }
            _ => anyhow::bail!("Expected function or variable"),
        };
        return Ok(tokens);
    }
}
