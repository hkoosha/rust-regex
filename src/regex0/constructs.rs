use error::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

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
            Self::LeftParen => "LeftParen",
            Self::RightParen => "RightParen",
            Self::Star => "Star",
            Self::Alt => "Alt",
            Self::Concat => "Concat",
            Self::Plus => "Plus",
            Self::QMark => "QMark",
            Self::Char(_) => "Char",
            Self::None => "None",
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Token::Char(c) => write!(f, "Token=[{}]", c),
            Token::None => write!(f, "Token[]"),
            _ => write!(f, "Token=[{}]", self.symbol().unwrap()),
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
    pub fn new(name: String) -> Self {
        Self {
            epsilon: vec![],
            transitions: HashMap::new(),
            name,
            is_end: false,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
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
