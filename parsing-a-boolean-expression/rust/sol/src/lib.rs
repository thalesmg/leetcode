struct Solution;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Negate,
    True,
    False,
}

#[derive(PartialEq, Debug)]
enum AST {
    Bool(bool),
    And(Vec<AST>),
    Or(Vec<AST>),
    Negate(Box<AST>),
}

#[derive(PartialEq, Debug)]
enum ParseError {
    StringTooShort,
    UnexpectedToken(Token, Token),
    InvalidSyntax,
}

struct ParserState<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> ParserState<'a> {
    pub fn new(tokens: &'a Vec<Token>, pos: usize) -> Self {
        ParserState { tokens, pos }
    }

    fn get_next_token(&mut self) -> Result<&'a Token, ParseError> {
        match self.tokens.get(self.pos) {
            None => Err(ParseError::StringTooShort),
            Some(token) => {
                self.pos += 1;
                Ok(token)
            }
        }
    }

    fn expect_next_token(&mut self, expected: Token) -> Result<(), ParseError> {
        let token = self.get_next_token()?;
        if token == &expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(token.clone(), expected))
        }
    }

    fn peek_next_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn parse_group(&mut self) -> Result<Vec<AST>, ParseError> {
        self.expect_next_token(Token::LParen)?;

        let mut exprs = vec![];
        while self.peek_next_token() != Some(&Token::RParen) {
            let expr = self.parse_next()?;
            exprs.push(expr);
        }
        if exprs.len() == 0 {
            return Err(ParseError::InvalidSyntax);
        }

        self.expect_next_token(Token::RParen)?;

        Ok(exprs)
    }

    fn parse_next(&mut self) -> Result<AST, ParseError> {
        let token = self.get_next_token()?;
        match token {
            Token::True => Ok(AST::Bool(true)),
            Token::False => Ok(AST::Bool(false)),
            Token::And => {
                let exprs = self.parse_group()?;
                Ok(AST::And(exprs))
            }
            Token::Or => {
                let exprs = self.parse_group()?;
                Ok(AST::Or(exprs))
            }
            Token::Negate => {
                self.expect_next_token(Token::LParen)?;
                let expr = self.parse_next()?;
                self.expect_next_token(Token::RParen)?;
                Ok(AST::Negate(Box::new(expr)))
            }
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl AST {
    pub fn eval(&self) -> bool {
        match self {
            AST::Bool(b) => *b,
            AST::Negate(b) => !b.eval(),
            AST::And(asts) => asts.iter().fold(true, |res, ast| res && ast.eval()),
            AST::Or(asts) => asts.iter().fold(false, |res, ast| res || ast.eval()),
        }
    }
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let lexed = Solution::lex(expression);
        let parsed = Solution::parse(lexed);
        parsed.unwrap().eval()
    }

    fn char_to_token(c: char) -> Option<Token> {
        match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '&' => Some(Token::And),
            '|' => Some(Token::Or),
            '!' => Some(Token::Negate),
            't' => Some(Token::True),
            'f' => Some(Token::False),
            _ => None,
        }
    }

    fn lex(raw: String) -> Vec<Token> {
        let mut out = vec![];

        for c in raw.chars() {
            if let Some(t) = Solution::char_to_token(c) {
                out.push(t);
            }
        }

        out
    }

    fn parse(tokens: Vec<Token>) -> Result<AST, ParseError> {
        let mut state = ParserState::new(&tokens, 0);

        state.parse_next()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        let input = "!(f)".to_string();
        assert_eq!(true, Solution::parse_bool_expr(input));
    }

    #[test]
    fn ex2() {
        let input = "|(f,t)".to_string();
        assert_eq!(true, Solution::parse_bool_expr(input));
    }

    #[test]
    fn ex3() {
        let input = "&(t,f)".to_string();
        assert_eq!(false, Solution::parse_bool_expr(input));
    }

    #[test]
    fn ex4() {
        let input = "|(&(t,f,t),!(t))".to_string();
        assert_eq!(false, Solution::parse_bool_expr(input));
    }
}
