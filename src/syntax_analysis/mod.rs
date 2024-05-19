use crate::{lexical_analysis::Token, syntax_analysis::translation_unit::TranslationUnit};

mod block_statement;
mod declaration;
mod expression;
mod parameter_definition;
mod statement;
mod translation_unit;
mod types;

pub(self) fn next(tokens: &[Token]) -> anyhow::Result<(Token, &[Token])> {
    match tokens {
        [] => anyhow::bail!("Expected token"),
        [token, ..] => Ok((token.to_owned(), &tokens[1..])),
    }
}

#[allow(unused_variables, dead_code)]
fn parse_some_thing<'a, T>(tokens: &'a [Token], target: &mut T) -> anyhow::Result<&'a [Token]> {
    unimplemented!()
}

pub(crate) fn parse(mut tokens: &[Token]) -> anyhow::Result<()> {
    let mut unit = None;
    tokens = TranslationUnit::parse(tokens, &mut unit)?;
    debug_assert!(
        tokens.is_empty(),
        "Token not empty after parsing translation unit"
    );
    tracing::info!("Parse result {unit:?}");
    Ok(())
}
