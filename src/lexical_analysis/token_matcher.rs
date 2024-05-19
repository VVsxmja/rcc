use super::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(super) struct TokenMatcher {
    pub(self) child: HashMap<char, TokenMatcher>,
    pub(self) current_token: Option<Token>,
}

impl TokenMatcher {
    pub(crate) fn new() -> Self {
        TokenMatcher {
            child: HashMap::new(),
            current_token: None,
        }
    }
    fn patch(&mut self, pattern: &[char], token: Token) -> Result<(), ()> {
        match pattern {
            [] => {
                if self.current_token.is_none() {
                    self.current_token = Some(token);
                    Ok(())
                } else {
                    Err(())
                }
            }
            [ch, ..] => match self.child.get_mut(ch) {
                Some(child) => child.patch(&pattern[1..], token),
                None => {
                    let mut child = TokenMatcher::new();
                    child.patch(&pattern[1..], token)?;
                    self.child.insert(ch.to_owned(), child);
                    Ok(())
                }
            },
        }
    }
    pub(crate) fn get_token(&self, input: &str) -> Option<(Token, usize)> {
        fn search(matcher: &TokenMatcher, input: &[char], depth: usize) -> Option<(Token, usize)> {
            match input {
                [] => matcher.current_token.to_owned().map(|token| (token, depth)),
                [ch, ..] => match matcher.child.get(ch) {
                    Some(child) => search(child, &input[1..], depth + 1),
                    None => matcher.current_token.to_owned().map(|token| (token, depth)),
                },
            }
        }
        let input: Box<[char]> = input.chars().collect();
        search(self, &input, 0)
    }
}

enum TokenMatcherBuilder {
    Ok(TokenMatcher),
    DuplicatePattern(String),
}

impl TokenMatcherBuilder {
    pub(crate) fn new() -> Self {
        TokenMatcherBuilder::Ok(TokenMatcher::new())
    }
    pub(crate) fn with(self, pattern: &str, token: Token) -> Self {
        match self {
            TokenMatcherBuilder::Ok(mut matcher) => {
                let pattern_as_slice: Box<[char]> = pattern.chars().collect();
                match matcher.patch(&pattern_as_slice, token) {
                    Err(_) => TokenMatcherBuilder::DuplicatePattern(pattern.to_string()),
                    Ok(_) => TokenMatcherBuilder::Ok(matcher),
                }
            }
            err => err,
        }
    }
    pub(crate) fn build(self) -> anyhow::Result<TokenMatcher> {
        match self {
            Self::Ok(matcher) => Ok(matcher),
            Self::DuplicatePattern(pattern) => anyhow::bail!("Duplicate pattern: {pattern}"),
        }
    }
}

lazy_static! {
    pub(super) static ref TOKEN_MATCHER: TokenMatcher = {
        match TokenMatcherBuilder::new()
            .with("(", Token::Symbol(Symbol::LeftParen))
            .with(")", Token::Symbol(Symbol::RightParen))
            .with("{", Token::Symbol(Symbol::LeftBrace))
            .with("}", Token::Symbol(Symbol::RightBrace))
            .with("+", Token::Symbol(Symbol::Plus))
            .with("-", Token::Symbol(Symbol::Minus))
            .with("*", Token::Symbol(Symbol::Star))
            .with("/", Token::Symbol(Symbol::Slash))
            .with("%", Token::Symbol(Symbol::Modulo))
            .with("==", Token::Symbol(Symbol::Equal))
            .with("!=", Token::Symbol(Symbol::NotEqual))
            .with("<", Token::Symbol(Symbol::Less))
            .with("<=", Token::Symbol(Symbol::LessEqual))
            .with(">", Token::Symbol(Symbol::More))
            .with(">=", Token::Symbol(Symbol::MoreEqual))
            .with("||", Token::Symbol(Symbol::Or))
            .with("&&", Token::Symbol(Symbol::And))
            .with("!", Token::Symbol(Symbol::Not))
            .with("=", Token::Symbol(Symbol::Assign))
            .with(",", Token::Symbol(Symbol::Comma))
            .with(";", Token::Symbol(Symbol::Semicolon))
            .with("int", Token::Keyword(Keyword::Int))
            .with("void", Token::Keyword(Keyword::Void))
            .with("if", Token::Keyword(Keyword::If))
            .with("else", Token::Keyword(Keyword::Else))
            .with("while", Token::Keyword(Keyword::While))
            .with("return", Token::Keyword(Keyword::Return))
            .with("true", Token::Constant(Constant::Int(1)))
            .with("false", Token::Constant(Constant::Int(0)))
            .build()
        {
            Ok(matcher) => matcher,
            Err(e) => panic!("[ Init Error ] {e}"),
        }
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn token_matcher_not_allow_duplicate() {
        let result = TokenMatcherBuilder::new()
            .with("duplicate", Token::Identifier("duplicate".to_string()))
            .with("duplicate", Token::Identifier("duplicate".to_string()))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn rcc_node_structure() {
        fn right_structure(current: &TokenMatcher) {
            if current.current_token.is_none() {
                assert!(!current.child.is_empty())
            }
            for (_, child) in &current.child {
                right_structure(child);
            }
        }
        right_structure(&TOKEN_MATCHER)
    }

    #[test]
    fn rcc_no_duplicate_token() {
        let mut all_tokens = HashSet::new();
        fn no_duplicate(current: &TokenMatcher, all_tokens: &mut HashSet<Token>) {
            if let Some(token) = &current.current_token {
                assert!(!all_tokens.contains(token));
                all_tokens.insert(token.to_owned());
            }
            for (_, child) in &current.child {
                no_duplicate(child, all_tokens);
            }
        }
        no_duplicate(&TOKEN_MATCHER, &mut all_tokens)
    }

    #[test]
    fn rcc_can_get_all_valid_token() {
        fn all_tested(tested: &HashSet<String>, token: String, current: &TokenMatcher) {
            if current.current_token.is_some() {
                println!("Testing if {token} is tested");
                assert!(tested.contains(&token))
            } else {
                for (ch, child) in &current.child {
                    all_tested(tested, format!("{token}{ch}"), child)
                }
            }
        }
        let tested = &mut HashSet::new();
        let mut test_token = |input: &str, target: Token| {
            println!("Testing {input}");
            tested.insert(input.to_string());
            assert!(matches!(TOKEN_MATCHER.get_token(input), Some((token, _)) if token == target))
        };
        test_token("(", Token::Symbol(Symbol::LeftParen));
        test_token(")", Token::Symbol(Symbol::RightParen));
        test_token("{", Token::Symbol(Symbol::LeftBrace));
        test_token("}", Token::Symbol(Symbol::RightBrace));
        test_token("+", Token::Symbol(Symbol::Plus));
        test_token("-", Token::Symbol(Symbol::Minus));
        test_token("*", Token::Symbol(Symbol::Star));
        test_token("/", Token::Symbol(Symbol::Slash));
        test_token("%", Token::Symbol(Symbol::Modulo));
        test_token("==", Token::Symbol(Symbol::Equal));
        test_token("!=", Token::Symbol(Symbol::NotEqual));
        test_token("<", Token::Symbol(Symbol::Less));
        test_token("<=", Token::Symbol(Symbol::LessEqual));
        test_token(">", Token::Symbol(Symbol::More));
        test_token(">=", Token::Symbol(Symbol::MoreEqual));
        test_token("||", Token::Symbol(Symbol::Or));
        test_token("&&", Token::Symbol(Symbol::And));
        test_token("!", Token::Symbol(Symbol::Not));
        test_token("=", Token::Symbol(Symbol::Assign));
        test_token(",", Token::Symbol(Symbol::Comma));
        test_token(";", Token::Symbol(Symbol::Semicolon));
        test_token("int", Token::Keyword(Keyword::Int));
        test_token("void", Token::Keyword(Keyword::Void));
        test_token("if", Token::Keyword(Keyword::If));
        test_token("else", Token::Keyword(Keyword::Else));
        test_token("while", Token::Keyword(Keyword::While));
        test_token("return", Token::Keyword(Keyword::Return));
        test_token("true", Token::Constant(Constant::Int(1)));
        test_token("false", Token::Constant(Constant::Int(0)));
        all_tested(tested, "".to_string(), &TOKEN_MATCHER)
    }
}