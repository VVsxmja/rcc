use crate::lexical_analysis::Token;

#[derive(Debug)]
pub(crate) struct Expression {}

impl Expression {
    #[allow(unused_variables)]
    pub(crate) fn parse<'a>(
        tokens: &'a [Token],
        target: &mut Option<Expression>,
    ) -> anyhow::Result<&'a [Token]> {
        unimplemented!()
    }
}