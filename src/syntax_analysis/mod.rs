use crate::{lexical_analysis::Token, syntax_analysis::translation_unit::TranslationUnit};

mod block_statement;
mod declaration;
mod expression;
mod parameter_definition;
mod statement;
mod translation_unit;
mod types;

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
