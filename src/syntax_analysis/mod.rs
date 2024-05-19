use crate::{
    lexical_analysis::{Token},
    syntax_analysis::{translation_unit::TranslationUnit, types::Type},
};

use self::{expression::Expression};

mod block_statement;
mod expression;
mod parameter_definition;
mod statement;
mod top_level;
mod translation_unit;
mod types;

#[derive(Debug)]
pub(crate) struct VariableDefinition(Type, String, Option<Expression>);

pub(crate) fn next(tokens: &[Token]) -> anyhow::Result<(Token, &[Token])> {
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
    let mut unit = Vec::new();
    tokens = TranslationUnit::parse(tokens, &mut unit)?;
    debug_assert!(
        tokens.is_empty(),
        "Token not empty after parsing translation unit"
    );
    tracing::info!("Parse result {unit:?}");
    Ok(())
}
