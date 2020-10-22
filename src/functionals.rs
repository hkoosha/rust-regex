use std::collections::HashMap;

use crate::constructs::Token;

pub struct Lexer {
    pattern: String,
    current: usize,
    symbols: HashMap<char, Token>,
}

impl Lexer {
    pub fn new(pattern: String) -> Lexer {
        return Lexer {
            pattern,
            current: 0,
            symbols: vec![
                ('(', Token::LeftParen),
                (')', Token::RightParen),
                ('*', Token::Star),
                ('|', Token::Alt),
                ('.', Token::Concat),
                ('+', Token::Plus),
                ('?', Token::QMark),
            ].into_iter().collect(),
        };
    }

    pub fn get_token(&mut self) -> Token {
        return if self.current < self.pattern.len() {
            let c = self.pattern.chars().nth(self.current).unwrap();
            self.current += 1;

            return if self.symbols.contains_key(&c) {
                self.symbols[&c]
            } else {
                Token::Char(c)
            };
        } else {
            Token::None
        };
    }
}

// ----------------------------------

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    lookahead: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let lookahead = lexer.get_token();
        Parser {
            lexer,
            tokens: vec![],
            lookahead,
        }
    }

    fn append(&mut self, t: Token) {
        self.tokens.append(&mut vec![t]);
    }

    fn consume(&mut self, token: &Token) -> Result<(), ParseError> {
        println!("consume({})", token);
        return if self.lookahead.name() == token.name() {
            self.lookahead = self.lexer.get_token();
            Ok(())
        } else {
            Err(ParseError {})
        };
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("token={}", token);
        }
    }

    pub fn parse(&mut self, print_tokens: bool) -> &Vec<Token> {
        println!("parse()");

        self.exp();

        if print_tokens {
            self.print_tokens();
        }
        return &self.tokens;
    }

    fn exp(&mut self) {
        println!("exp()");
        self.term();
        if let Token::Alt = self.lookahead {
            let t = self.lookahead;
            self.consume(&Token::Alt).unwrap();
            self.exp();
            self.append(t);
        }
    }

    fn term(&mut self) {
        println!("term()");
        self.factor();
        match self.lookahead {
            Token::Alt | Token::RightParen | Token::None => {}
            _ => {
                self.term();
                self.append(Token::Concat);
            }
        }
    }

    fn factor(&mut self) {
        println!("factor()");
        self.primary();
        if let Token::Star | Token::Plus | Token::QMark = self.lookahead {
            self.append(self.lookahead);
            let la = (&self.lookahead).clone();
            self.consume(&la).unwrap();
        }
    }

    fn primary(&mut self) {
        println!("primary()");
        match self.lookahead {
            Token::LeftParen => {
                println!("primary() -> LeftParen");
                self.consume(&Token::LeftParen).unwrap();
                self.exp();
                self.consume(&Token::RightParen).unwrap();
            }
            Token::Char(v) => {
                println!("primary() -> char : {}", v);
                self.append(self.lookahead);
                self.consume(&Token::Char(0 as char)).unwrap();
            }
            _ => {
                println!("primary() -> none! panic?");
            }
        }
    }
}
