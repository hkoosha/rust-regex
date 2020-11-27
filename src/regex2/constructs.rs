use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type RNfaState = Rc<RefCell<NfaState>>;

pub struct NfaState {
    name: usize,
    epsilon: Vec<RNfaState>,
    delta: HashMap<char, RNfaState>,
}

impl NfaState {
    pub fn new(name: usize) -> Self {
        NfaState {
            name,
            epsilon: vec![],
            delta: HashMap::new(),
        }
    }

    pub fn add_epsilon(&mut self, state: RNfaState) {
        self.epsilon.push(state);
    }

    pub fn add_delta(&mut self, symbol: char, state: RNfaState) {
        self.delta.insert(symbol, state);
    }
}

impl Hash for NfaState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for NfaState {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for NfaState {}

impl Display for NfaState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "NfaState[name={}]", self.name)
    }
}
