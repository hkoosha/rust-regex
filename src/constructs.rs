use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
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

    pub fn symbol(&self) -> char {
        match &self {
            Token::LeftParen => '(',
            Token::RightParen => ')',
            Token::Star => '*',
            Token::Alt => '|',
            Token::Concat => '.',
            Token::Plus => '+',
            Token::QMark => '?',
            _ => panic!("does not have a symbol associated with it."),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name = match self {
            Token::Char(c) => format!("Char={}", c),
            _ => self.name().to_string(),
        };
        write!(f, "Token[{}]", name)
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
