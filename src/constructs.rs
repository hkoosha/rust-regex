use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::error;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Star,
    Alt,
    Concat,
    Plus,
    QMark,
    Char(char),
    None,
}

impl Token {
    pub fn name(&self) -> &'static str {
        match &self {
            Token::LeftParen => "LeftParen",
            Token::RightParen => "RightParen",
            Token::Star => "Star",
            Token::Alt => "Alt",
            Token::Concat => "Concat",
            Token::Plus => "Plus",
            Token::QMark => "QMark",
            Token::Char(_) => "Char",
            Token::None => "None",
        }
    }

    pub fn symbol(&self) -> Option<char> {
        match &self {
            Token::LeftParen => Some('('),
            Token::RightParen => Some(')'),
            Token::Star => Some('*'),
            Token::Alt => Some('|'),
            Token::Concat => Some('.'),
            Token::Plus => Some('+'),
            Token::QMark => Some('?'),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Token::Char(c) => write!(f, "Token=[{}]", c),
            Token::None => write!(f, "Token[]"),
            _ => write!(f, "Token=[{}]", self.symbol().unwrap())
        }
    }
}

// ----------------------------------

pub type SState = Rc<RefCell<State>>;

pub struct State {
    pub epsilon: Vec<SState>,
    pub transitions: HashMap<char, SState>,
    pub name: String,
    pub is_end: bool,
}

impl State {
    pub fn new(name: String) -> State {
        State {
            epsilon: vec![],
            transitions: HashMap::new(),
            name,
            is_end: false,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "State[{}]", self.name)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}


// ----------------------------------

#[derive(Debug)]
pub struct ParseError {
    error: String,
}

/// Errors by parser
impl ParseError {
    pub fn new(error: String) -> ParseError {
        ParseError {
            error,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ParseError[{}]", &self.error)
    }
}

impl error::Error for ParseError {}


#[derive(Debug)]
pub struct NfaConstructionError {
    error: String,
}

impl NfaConstructionError {
    pub fn new(error: String) -> NfaConstructionError {
        NfaConstructionError {
            error,
        }
    }
}

impl Display for NfaConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NfaConstructionError[{}]", &self.error)
    }
}

impl error::Error for NfaConstructionError {}

