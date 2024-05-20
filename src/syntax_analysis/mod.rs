use crate::{lexical_analysis::Token, syntax_analysis::translation_unit::TranslationUnit};

pub(crate) mod block_statement;
pub(crate) mod declaration;
pub(crate) mod expression;
pub(crate) mod parameter_definition;
pub(crate) mod statement;
pub(crate) mod translation_unit;
pub(crate) mod types;

pub(self) fn next(tokens: &[Token]) -> anyhow::Result<(&[Token], Token)> {
    match tokens {
        [] => anyhow::bail!("Expected token"),
        [token, ..] => Ok((&tokens[1..], token.to_owned())),
    }
}

pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<TranslationUnit> {
    let (tokens, unit) = TranslationUnit::parse(tokens)?;
    debug_assert!(
        tokens.is_empty(),
        "Token not empty after parsing translation unit"
    );
    Ok(unit)
}
