use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::regex1::parser::{Parser, TreeNode};

const INITIAL_CAPACITY: usize = 8;

type State = Rc<RefCell<_State>>;

struct _State {
    name: usize,
    is_end: bool,
    transitions: HashMap<char, State>,
    epsilon_transitions: Vec<State>,
}

impl _State {
    fn new(name: usize, is_end: bool) -> _State {
        _State {
            name,
            is_end,
            transitions: HashMap::with_capacity(INITIAL_CAPACITY),
            epsilon_transitions: Vec::with_capacity(INITIAL_CAPACITY),
        }
    }

    fn from_start(name: usize) -> State {
        Self::new(name, false).into_cell()
    }

    fn from_end(name: usize) -> State {
        Self::new(name, true).into_cell()
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

impl PartialEq for _State {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for _State {}

impl PartialOrd for _State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for _State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Display for _State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State[name={}, is_end={}, transitions={}, epsilons={}]",
            self.name,
            self.is_end,
            self.transitions.len(),
            self.epsilon_transitions.len()
        )
    }
}

impl Debug for _State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Hash for _State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// -----------------------------------------------------------------------------

type Namer = Rc<RefCell<dyn FnMut() -> usize>>;

fn new_namer() -> Namer {
    let mut name_holder = 0usize;
    let name = move || {
        let new_name = name_holder;
        name_holder += 1;
        new_name
    };
    Rc::new(RefCell::new(name))
}

#[derive(Debug)]
pub struct NFA {
    start: State,
    end: State,
}

impl NFA {
    fn new(start: State, end: State) -> NFA {
        NFA { start, end }
    }

    fn from_epsilon(namer: Namer) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());
        start.borrow_mut().add_epsilon_transition(end.clone());
        Self::new(start, end)
    }

    fn from_symbol(namer: Namer, symbol: char) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());
        start.borrow_mut().add_transition(end.clone(), symbol);
        Self::new(start, end)
    }

    // -------------

    fn concat(&mut self, second: NFA) -> NFA {
        self.end.borrow_mut().add_epsilon_transition(second.start);
        self.end.borrow_mut().is_end = false;
        Self::new(self.start.clone(), second.end)
    }

    fn union(&mut self, namer: Namer, second: NFA) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());

        start
            .borrow_mut()
            .add_epsilon_transition(self.start.clone());
        start
            .borrow_mut()
            .add_epsilon_transition(second.start.clone());

        self.end.borrow_mut().add_epsilon_transition(end.clone());
        self.end.borrow_mut().is_end = false;

        second.end.borrow_mut().add_epsilon_transition(end.clone());
        second.end.borrow_mut().is_end = false;

        NFA::new(start, end)
    }

    fn kleen_closure(self, namer: Namer) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());

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

    fn zero_or_one(self, namer: Namer) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());

        start.borrow_mut().add_epsilon_transition(end.clone());
        start
            .borrow_mut()
            .add_epsilon_transition(self.start.clone());

        self.end.borrow_mut().add_epsilon_transition(end.clone());

        self.end.borrow_mut().is_end = false;

        NFA::new(start, end)
    }

    fn one_or_more(self, namer: Namer) -> NFA {
        let start = _State::from_start(namer.borrow_mut()());
        let end = _State::from_end(namer.borrow_mut()());

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

impl PartialEq for NFA {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end && self.start == other.start
    }
}

impl Eq for NFA {}

impl Display for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NFA<start={}, end={}>",
            self.start.borrow(),
            self.end.borrow()
        )
    }
}

impl Hash for NFA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.borrow().name.hash(state);
        self.end.borrow().name.hash(state);
    }
}

// ----------

pub fn postfix_to_nfa(regex: &str) -> Result<NFA, String> {
    let namer = new_namer();

    if regex.is_empty() {
        return Ok(NFA::from_epsilon(namer.clone()));
    }

    let mut stack: Vec<NFA> = Vec::new();
    for token in regex.chars() {
        match token {
            '*' => {
                if stack.is_empty() {
                    return Err(
                        "stack is empty while expecting at least one element for operation: *"
                            .to_string(),
                    );
                }
                let nfa = stack.pop().unwrap();
                stack.push(nfa.kleen_closure(namer.clone()))
            }
            '?' => {
                if stack.is_empty() {
                    return Err(
                        "stack is empty while expecting at least one element for operation: ?"
                            .to_string(),
                    );
                }
                let nfa = stack.pop().unwrap();
                stack.push(nfa.zero_or_one(namer.clone()));
            }
            '+' => {
                if stack.is_empty() {
                    return Err(
                        "stack is empty while expecting at least one element for operation: +"
                            .to_string(),
                    );
                }
                let nfa = stack.pop().unwrap();
                stack.push(nfa.one_or_more(namer.clone()));
            }
            '|' => {
                if stack.len() < 2 {
                    return Err(format!(
                        "stack has less than two elements, while expecting at least two element \
                        for operation: `|`, number of elements in stack: {}",
                        stack.len()
                    ));
                }
                let right = stack.pop().unwrap();
                let mut left = stack.pop().unwrap();
                stack.push(left.union(namer.clone(), right));
            }
            '.' => {
                if stack.len() < 2 {
                    return Err(format!(
                        "stack has less than two elements, while expecting at least two element, \
                        for operation: `.`, number of elements in stack: {}",
                        stack.len()
                    ));
                }
                let right = stack.pop().unwrap();
                let mut left = stack.pop().unwrap();
                stack.push(left.concat(right));
            }
            _ => {
                stack.push(NFA::from_symbol(namer.clone(), token));
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

// ----------

fn parse_tree_to_nfa(root: &TreeNode, namer: Namer) -> Result<NFA, String> {
    match root.label.as_str() {
        "Expr" => {
            let mut term = parse_tree_to_nfa(&root.children[0], namer.clone())?;
            match root.children.len() {
                // Expr -> Term '|' Expr
                3 => Ok(term.union(
                    namer.clone(),
                    parse_tree_to_nfa(&root.children[2], namer.clone())?,
                )),
                _ => Ok(term),
            }
        }
        "Term" => {
            let mut factor = parse_tree_to_nfa(&root.children[0], namer.clone())?;
            match root.children.len() {
                2 => Ok(factor.concat(parse_tree_to_nfa(&root.children[1], namer)?)),
                _ => Ok(factor),
            }
        }
        "Factor" => {
            let atom = parse_tree_to_nfa(&root.children[0], namer.clone())?;
            match root.children.len() {
                2 => match root.children[1].label.as_str() {
                    "*" => Ok(atom.kleen_closure(namer)),
                    "+" => Ok(atom.one_or_more(namer)),
                    "?" => Ok(atom.zero_or_one(namer)),
                    _ => Ok(atom),
                },
                _ => Ok(atom),
            }
        }
        "Atom" => match root.children.len() {
            3 => parse_tree_to_nfa(&root.children[1], namer),
            _ => parse_tree_to_nfa(&root.children[0], namer),
        },
        "Char" => match root.children.len() {
            2 => Ok(NFA::from_symbol(
                namer,
                root.children[1].label.chars().next().unwrap(),
            )),
            _ => Ok(NFA::from_symbol(
                namer,
                root.children[0].label.chars().next().unwrap(),
            )),
        },
        _ => Err(format!("unrecognized node label: {}", root.label)),
    }
}

pub fn infix_to_nfa(regex: &str) -> Result<NFA, String> {
    let namer = new_namer();

    if regex.is_empty() {
        return Ok(NFA::from_epsilon(namer));
    }

    let parse_tree = Parser::new(regex.to_string()).parse()?;
    parse_tree_to_nfa(&parse_tree, namer)
}

// ----------

fn add_next_state(state: &State, next_states: &mut Vec<State>, visited: Rc<RefCell<Vec<usize>>>) {
    if state.borrow().epsilon_transitions.is_empty() {
        next_states.push(state.clone());
    } else {
        for s in &state.borrow().epsilon_transitions {
            if !visited.borrow().contains(&s.borrow().name) {
                visited.borrow_mut().push(s.borrow().name);
                add_next_state(s, next_states, visited.clone())
            }
        }
    }
}

pub fn recognize(nfa: &NFA, word: &str) -> bool {
    let mut current_states: Vec<State> = vec![];

    // The initial set of current states is either the start state or
    // the set of states reachable by epsilon transitions from the start state.
    add_next_state(
        &nfa.start,
        &mut current_states,
        Rc::new(RefCell::new(vec![])),
    );

    for symbol in word.chars() {
        let mut next_states: Vec<State> = vec![];
        for state in &current_states {
            if let Some(next_state) = state.borrow().transitions.get(&symbol) {
                add_next_state(next_state, &mut next_states, Rc::new(RefCell::new(vec![])));
            }
        }

        current_states = next_states;
    }

    current_states.into_iter().any(|s| s.borrow().is_end)
}
