use crate::{lexical_analysis::Token, syntax_analysis::next};

use super::types::Type;

#[derive(Debug)]
pub(crate) struct ParameterDefinition(Type, String);

impl ParameterDefinition {
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Vec<Self>,
    ) -> anyhow::Result<&'a [Token]> {
        let mut decl_type = None;
        let tokens = Type::parse(tokens, &mut decl_type)?;
        let Some(decl_type) = decl_type else {
            anyhow::bail!("Expected type");
        };
        tracing::trace!("Parameter type: {decl_type:?}");
        let (Token::Identifier(id), tokens) = next(tokens)? else {
            anyhow::bail!("Expected identifier");
        };
        tracing::trace!("Parameter id: {id:?}");
        target.push(ParameterDefinition(decl_type, id));
        Ok(tokens)
    }
}
