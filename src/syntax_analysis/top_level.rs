use crate::lexical_analysis::{Keyword, Symbol, Token};

use super::{
    block_statement::BlockStatement, expression::Expression, next,
    parameter_definition::ParameterDefinition, types::Type, VariableDefinition,
};

#[derive(Debug)]
pub(crate) enum TopLevel {
    Variable(VariableDefinition),
    Function(
        Type,
        String,
        Vec<ParameterDefinition>,
        Option<BlockStatement>,
    ),
}

impl TopLevel {
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Vec<Self>,
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
                        target.push(TopLevel::Function(decl_type, id, params, None));
                        remain_tokens
                    }
                    _ => {
                        let mut block_stmt = None;
                        tokens = BlockStatement::parse(tokens, &mut block_stmt)?;
                        let Some(block_stmt) = block_stmt else {
                            unreachable!("parse_block_statement exit without error nor result");
                        };
                        target.push(TopLevel::Function(decl_type, id, params, Some(block_stmt)));
                        tokens
                    }
                }
            }
            Token::Symbol(Symbol::Semicolon) => {
                let variable_definition = VariableDefinition(decl_type, id, None);
                tracing::trace!("Global Variable definition: {variable_definition:?}");
                target.push(TopLevel::Variable(variable_definition));
                tokens
            }
            Token::Symbol(Symbol::Assign) => {
                let mut expr = None;
                let tokens = Expression::parse(tokens, &mut expr)?;
                let Some(expr) = expr else {
                    unreachable!("parse_expression exit without error nor result");
                };
                let variable_declaration = VariableDefinition(decl_type, id, Some(expr));
                tracing::trace!("Global Variable Declaration: {variable_declaration:?}");
                target.push(TopLevel::Variable(variable_declaration));
                let (Token::Symbol(Symbol::Semicolon), tokens) = next(tokens)? else {
                    anyhow::bail!("Expected semicolon");
                };
                tokens
            }
            _ => anyhow::bail!("Expected function or global variable, found {token_after_id:?}"),
        };
        return Ok(tokens);
    }
}
