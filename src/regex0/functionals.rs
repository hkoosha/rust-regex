use std::collections::HashMap;

use crate::regex0::constructs::Token;

pub struct Lexer {
    pattern: String,
    current: usize,
    symbols: HashMap<char, Token>,
}

impl Lexer {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            current: 0,
            symbols: vec![
                Token::LeftParen,
                Token::RightParen,
                Token::Star,
                Token::Alt,
                Token::Concat,
                Token::Plus,
                Token::QMark,
            ]
            .into_iter()
            .map(|it| (it.symbol().unwrap(), it))
            .collect(),
        }
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

pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
    lookahead: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let lookahead = lexer.get_token();
        Self {
            lexer,
            tokens: vec![],
            lookahead,
        }
    }

    fn print_tokens(&self) {
        for token in &self.tokens {
            eprintln!("{}::{:03} :: => token={}", file!(), line!(), token);
        }
    }

    fn append(&mut self, t: Token) {
        self.tokens.append(&mut vec![t]);
    }

    fn consume(&mut self, token: &Token) -> Result<(), String> {
        eprintln!("{}::{:03} :: consume({})", file!(), line!(), token);
        return if self.lookahead.name() == token.name() {
            self.lookahead = self.lexer.get_token();
            Ok(())
        } else {
            Err(format!(
                "was expecting={} but got={}",
                token, self.lookahead
            ))
        };
    }

    pub fn parse(&mut self, print_tokens: bool) -> Result<&Vec<Token>, String> {
        eprintln!("{}::{:03} :: parse()", file!(), line!());
        self.exp()?;

        if print_tokens {
            self.print_tokens();
        }

        Ok(&self.tokens)
    }

    fn exp(&mut self) -> Result<(), String> {
        eprintln!("{}::{:03} :: exp()", file!(), line!());
        self.term()?;

        if let Token::Alt = self.lookahead {
            let t = self.lookahead;
            self.consume(&Token::Alt)?;
            self.exp()?;
            self.append(t);
        }

        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        eprintln!("{}::{:03} :: term()", file!(), line!());
        self.factor()?;

        match self.lookahead {
            Token::Alt | Token::RightParen | Token::None => {}
            _ => {
                self.term()?;
                self.append(Token::Concat);
            }
        }

        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        eprintln!("{}::{:03} :: factor()", file!(), line!());
        self.primary()?;

        if let Token::Star | Token::Plus | Token::QMark = self.lookahead {
            self.append(self.lookahead);
            let la = self.lookahead;
            self.consume(&la)?;
        }

        Ok(())
    }

    fn primary(&mut self) -> Result<(), String> {
        eprintln!("{}::{:03} :: primary()", file!(), line!());

        match self.lookahead {
            Token::LeftParen => {
                eprintln!("{}::{:03} :: primary() -> LeftParen", file!(), line!());
                self.consume(&Token::LeftParen)?;
                self.exp()?;
                self.consume(&Token::RightParen)?;
                Ok(())
            }
            Token::Char(v) => {
                eprintln!("{}::{:03} :: primary() -> char : {}", file!(), line!(), v);
                self.append(self.lookahead);
                self.consume(&Token::Char(0 as char))?;
                Ok(())
            }
            _ => Err(format!(
                "was not expecting this token type in primary() : {}",
                self.lookahead
            )),
        }
    }
}
