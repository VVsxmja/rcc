use crate::{lexical_analysis::Token, syntax_analysis::next};

use super::types::Type;

#[derive(Debug)]
pub struct ParameterDefinition(pub Type, pub String);

impl ParameterDefinition {
    pub fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, param_type) = Type::parse(tokens)?;
        if matches!(param_type, Type::Void) {
            anyhow::bail!("Parameter cannot be void");
        }
        let (tokens, Token::Identifier(param_name)) = next(tokens)? else {
            anyhow::bail!("Expected identifier");
        };
        Ok((tokens, ParameterDefinition(param_type, param_name)))
    }
}
