use crate::lexical_analysis::{Constant, Symbol, Token};

use super::next;

#[derive(Debug)]
pub(crate) enum Expression {
    Paren(Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    PrefixUnary(PrefixUnaryOperator, Box<Expression>),
    RefOrCall(RefOrCall),
    Constant(Constant),
}
#[derive(Debug)]
pub(crate) enum RefOrCall {
    FunctionCall(String, Vec<Expression>),
    Variable(String),
}

#[derive(Debug)]
pub(crate) enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Assign,
    Comma,
}

impl BinaryOperator {
    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        use BinaryOperator::*;
        let (tokens, token) = next(tokens)?;
        let op = match token {
            Token::Symbol(Symbol::Plus) => Plus,
            Token::Symbol(Symbol::Minus) => Minus,
            Token::Symbol(Symbol::Star) => Multiply,
            Token::Symbol(Symbol::Slash) => Divide,
            Token::Symbol(Symbol::Less) => Less,
            Token::Symbol(Symbol::LessEqual) => LessEqual,
            Token::Symbol(Symbol::Greater) => Greater,
            Token::Symbol(Symbol::GreaterEqual) => GreaterEqual,
            Token::Symbol(Symbol::EqualEqual) => Equal,
            Token::Symbol(Symbol::ExclaimEqual) => NotEqual,
            Token::Symbol(Symbol::Equal) => Assign,
            _ => anyhow::bail!("Expected binary operator"),
        };
        Ok((tokens, op))
    }
}

#[derive(Debug)]
pub(crate) enum PrefixUnaryOperator {
    Plus,
    Minus,
    Not,
}

impl PrefixUnaryOperator {
    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        use PrefixUnaryOperator::*;
        let (tokens, token) = next(tokens)?;
        let op = match token {
            Token::Symbol(Symbol::Plus) => Plus,
            Token::Symbol(Symbol::Minus) => Minus,
            Token::Symbol(Symbol::Exclaim) => Not,
            _ => anyhow::bail!("Expected prefix unary operator"),
        };
        Ok((tokens, op))
    }
}

/// https://en.cppreference.com/w/c/language/operator_precedence
trait Operator {
    fn precedence(&self) -> usize;
    fn right_associative(&self) -> bool;
}

impl Operator for BinaryOperator {
    fn precedence(&self) -> usize {
        use BinaryOperator::*;
        match self {
            Plus => 4,
            Minus => 4,
            Multiply => 3,
            Divide => 3,
            Less => 6,
            LessEqual => 6,
            Greater => 6,
            GreaterEqual => 6,
            Equal => 7,
            NotEqual => 7,
            Assign => 14,
            Comma => 15,
        }
    }

    fn right_associative(&self) -> bool {
        use BinaryOperator::*;
        match self {
            Plus => false,
            Minus => false,
            Multiply => false,
            Divide => false,
            Less => false,
            LessEqual => false,
            Greater => false,
            GreaterEqual => false,
            Equal => false,
            NotEqual => false,
            Assign => true,
            Comma => false,
        }
    }
}

impl Operator for PrefixUnaryOperator {
    fn precedence(&self) -> usize {
        use PrefixUnaryOperator::*;
        match self {
            Plus => 2,
            Minus => 2,
            Not => 2,
        }
    }

    fn right_associative(&self) -> bool {
        use PrefixUnaryOperator::*;
        match self {
            Plus => true,
            Minus => true,
            Not => true,
        }
    }
}

impl Expression {
    fn parse_ref_or_call(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, Token::Identifier(identifier)) = next(tokens)? else {
            anyhow::bail!("Expect identifier");
        };
        if let (mut tokens, Token::Symbol(Symbol::LeftParen)) = next(tokens)? {
            let mut params = Vec::new();
            match tokens {
                [Token::Symbol(Symbol::RightParen), remain @ ..] => Ok((
                    remain,
                    Expression::RefOrCall(RefOrCall::FunctionCall(identifier, params)),
                )),
                _ => loop {
                    let (remain, param) = Expression::parse(tokens)?;
                    params.push(param);
                    tokens = match remain {
                        [Token::Symbol(Symbol::RightParen), remain @ ..] => {
                            break Ok((
                                remain,
                                Expression::RefOrCall(RefOrCall::FunctionCall(identifier, params)),
                            ))
                        }
                        [Token::Symbol(Symbol::Comma), remain @ ..] => remain,
                        _ => anyhow::bail!("Expected \")\" or \",\""),
                    }
                },
            }
        } else {
            Ok((
                tokens,
                Expression::RefOrCall(RefOrCall::Variable(identifier)),
            ))
        }
    }
    fn parse_constant(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, Token::Constant(constant)) = next(tokens)? else {
            anyhow::bail!("Expect constant");
        };
        Ok((tokens, Expression::Constant(constant)))
    }
    fn parse_paren(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, Token::Symbol(Symbol::LeftParen)) = next(tokens)? else {
            anyhow::bail!("Expect left parentheses");
        };
        let (tokens, inner) = Expression::parse(tokens)?;
        let (tokens, Token::Symbol(Symbol::RightParen)) = next(tokens)? else {
            anyhow::bail!("Expect right parentheses");
        };
        Ok((tokens, Expression::Paren(Box::new(inner))))
    }

    fn parse_unary_operator(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        if let Ok((tokens, unary_op)) = PrefixUnaryOperator::parse(tokens) {
            let (tokens, operand) = Expression::parse_unary_operator(tokens)?;
            Ok((tokens, Expression::PrefixUnary(unary_op, Box::new(operand))))
        } else {
            Expression::parse_primary(tokens)
        }
    }

    fn parse_primary(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        match tokens {
            [Token::Identifier(_), ..] => Expression::parse_ref_or_call(tokens),
            [Token::Constant(_), ..] => Expression::parse_constant(tokens),
            [Token::Symbol(Symbol::LeftParen), ..] => Expression::parse_paren(tokens),
            _ => anyhow::bail!("Expected expression"),
        }
    }

    fn parse_rhs_of_binary(
        mut tokens: &[Token],
        mut lhs: Expression,
        max_precedence: usize,
    ) -> anyhow::Result<(&[Token], Self)> {
        loop {
            let Ok((remain, bin_op)) = BinaryOperator::parse(tokens) else {
                break Ok((tokens, lhs));
            };
            let right_assiciative = bin_op.right_associative();
            let precedence = bin_op.precedence();
            if precedence > max_precedence {
                break Ok((tokens, lhs));
            }
            let (remain, mut rhs) = Expression::parse_primary(remain)?;
            let Ok((_, next_bin_op)) = BinaryOperator::parse(remain) else {
                let expr = Expression::Binary(Box::new(lhs), bin_op, Box::new(rhs));
                break Ok((remain, expr));
            };
            let next_precedence = next_bin_op.precedence();
            if precedence > next_precedence || (precedence == next_precedence && right_assiciative)
            {
                let next_max_precedence = if right_assiciative {
                    precedence
                } else {
                    precedence - 1
                };
                let (remain, new_rhs) =
                    Expression::parse_rhs_of_binary(remain, rhs, next_max_precedence)?;
                rhs = new_rhs;
                tokens = remain;
            } else {
                tokens = remain;
            }
            lhs = Expression::Binary(Box::new(lhs), bin_op, Box::new(rhs));
        }
    }

    fn parse_binary_operator(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        let (tokens, lhs) = Expression::parse_unary_operator(tokens)?;
        Expression::parse_rhs_of_binary(tokens, lhs, BinaryOperator::Comma.precedence())
    }

    pub(crate) fn parse(tokens: &[Token]) -> anyhow::Result<(&[Token], Self)> {
        Expression::parse_binary_operator(tokens)
    }
}
