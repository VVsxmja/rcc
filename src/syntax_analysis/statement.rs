use crate::lexical_analysis::Token;

use super::{block_statement::BlockStatement, expression::Expression, VariableDefinition};

#[derive(Debug)]
pub(crate) enum Statement {
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Block(BlockStatement),
    LocalVariable(VariableDefinition),
    Expression(Expression),
}

impl Statement {
    #[allow(unused_variables)]
    pub(crate) fn parse<'a>(tokens: &'a [Token], target: &mut Vec<Statement>) -> anyhow::Result<&'a [Token]> {
        unimplemented!()
    }
}