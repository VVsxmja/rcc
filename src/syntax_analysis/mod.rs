use crate::{lexical_analysis::Token, syntax_analysis::translation_unit::TranslationUnit};

pub mod block_statement;
pub mod declaration;
pub mod expression;
pub mod parameter_definition;
pub mod statement;
pub mod translation_unit;
pub mod types;

fn next(tokens: &[Token]) -> anyhow::Result<(&[Token], Token)> {
    match tokens {
        [] => anyhow::bail!("Expected token"),
        [token, remain @ ..] => Ok((remain, token.to_owned())),
    }
}

pub fn parse(tokens: &[Token]) -> anyhow::Result<TranslationUnit> {
    let (tokens, unit) = TranslationUnit::parse(tokens)?;
    debug_assert!(
        tokens.is_empty(),
        "Token not empty after parsing translation unit"
    );
    Ok(unit)
}
