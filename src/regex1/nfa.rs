use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const INITIAL_CAPACITY: usize = 8;

type State = Rc<RefCell<_State>>;

#[derive(Debug)]
struct _State {
    is_end: bool,
    transitions: HashMap<char, State>,
    epsilon_transitions: Vec<State>,
}

impl _State {
    fn new(is_end: bool) -> _State {
        _State {
            is_end,
            transitions: HashMap::with_capacity(INITIAL_CAPACITY),
            epsilon_transitions: Vec::with_capacity(INITIAL_CAPACITY),
        }
    }

    fn from_start() -> State {
        Self::new(false).into_cell()
    }

    fn from_end() -> State {
        Self::new(true).into_cell()
    }

    fn into_cell(self) -> Rc<RefCell<_State>> {
        Rc::new(RefCell::new(self))
    }

    fn add_epsilon_transition(&mut self, to: State) {
        self.epsilon_transitions.push(to);
    }

    fn add_transition(&mut self, to: State, symbol: char) {
        self.transitions.insert(symbol, to);
    }
}

// ------

pub struct NFA {
    start: State,
    end: State,
}

impl NFA {
    fn new(start: State, end: State) -> NFA {
        NFA { start, end }
    }

    fn from_epsilon() -> NFA {
        let start = _State::from_start();
        let end = _State::from_end();
        start.borrow_mut().add_epsilon_transition(end.clone());
        Self::new(start, end)
    }

    fn from_symbol(symbol: char) -> NFA {
        let start = _State::from_start();
        let end = _State::from_end();
        start.borrow_mut().add_transition(end.clone(), symbol);
        Self::new(start, end)
    }

    fn concat(&self, second: NFA) -> NFA {
        self.end.borrow_mut().add_epsilon_transition(second.start);
        self.end.borrow_mut().is_end = false;
        Self::new(self.start.clone(), second.end)
    }

    fn union(&self, second: NFA) -> NFA {
        let start = _State::from_start();
        start
            .borrow_mut()
            .add_epsilon_transition(self.start.clone());
        start
            .borrow_mut()
            .add_epsilon_transition(second.start.clone());

        let end = _State::from_end();

        self.end.borrow_mut().add_epsilon_transition(end.clone());
        second.end.borrow_mut().add_epsilon_transition(end.clone());
        second.end.borrow_mut().is_end = false;

        NFA::new(start, end)
    }

    fn kleen_closure(&self) -> NFA {
        let start = _State::from_start();
        let end = _State::from_end();

        start.borrow_mut().add_epsilon_transition(end.clone());
        start
            .borrow_mut()
            .add_epsilon_transition(self.start.clone());

        self.end.borrow_mut().add_epsilon_transition(end.clone());
        self.end
            .borrow_mut()
            .add_epsilon_transition(self.start.clone());
        self.end.borrow_mut().is_end = false;

        NFA::new(start, end)
    }
}

fn zero_or_one(nfa: NFA) -> NFA {
    let start = _State::from_start();
    let end = _State::from_end();

    start.borrow_mut().add_epsilon_transition(end.clone());
    start.borrow_mut().add_epsilon_transition(nfa.start.clone());

    nfa.end.borrow_mut().add_epsilon_transition(end.clone());
    nfa.end.borrow_mut().is_end = false;

    NFA::new(start, end)
}

fn one_or_more(nfa: NFA) -> NFA {
    let start = _State::from_start();
    let end = _State::from_end();
    start.borrow_mut().add_epsilon_transition(nfa.start.clone());
    nfa.end.borrow_mut().add_epsilon_transition(end.clone());
    nfa.end
        .borrow_mut()
        .add_epsilon_transition(nfa.start.clone());
    nfa.end.borrow_mut().is_end = false;

    NFA::new(start, end)
}

pub fn postfix_to_nfa(regex: &str) -> Result<NFA, String> {
    if regex.is_empty() {
        return Ok(NFA::from_epsilon());
    }

    let mut stack: Vec<NFA> = Vec::new();

    for token in regex.chars() {
        match token {
            '*' => {
                if stack.is_empty() {
                    return Err("stack is empty while expecting at least one element".to_string());
                }
                let nfa = stack.pop().unwrap();
                stack.push(nfa.kleen_closure())
            }
            '?' => {
                if stack.is_empty() {
                    return Err("stack is empty while expecting at least one element".to_string());
                }
                let nfa = stack.pop().unwrap();
                stack.push(zero_or_one(nfa));
            }
            '+' => {
                if stack.is_empty() {
                    return Err("stack is empty while expecting at least one element".to_string());
                }
                let nfa = stack.pop().unwrap();
                stack.push(one_or_more(nfa));
            }
            '|' => {
                if stack.len() < 2 {
                    return Err(
                        "stack is empty or contains only one element while expecting at least two element"
                            .to_string(),
                    );
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.union(right));
            }
            '.' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.concat(right));
            }
            _ => {
                stack.push(NFA::from_symbol(token));
            }
        }
    }

    if stack.len() != 1 {
        Err(format!(
            "expecting exactly one element in NFA stack, but got: {}",
            stack.len()
        ))
    } else {
        Ok(stack.pop().unwrap())
    }
}
