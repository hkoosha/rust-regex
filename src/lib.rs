use std::cell::RefCell;
use std::rc::Rc;

use crate::constructs::{NfaConstructionError, SState, State, Token};
use crate::functionals::{Lexer, Parser};
use std::error::Error;

mod constructs;
mod functionals;

// ******* BE WARNED !!! There are mem leaks in the following code! *******

pub struct NFA {
    start: SState,
    end: SState,
}

fn add_state(state: &SState, states: &mut Vec<SState>) {
    // Already contains this state? skip.
    for s in states.iter() {
        if s == state {
            return;
        }
    }

    let v = Rc::clone(state);
    states.append(&mut vec![v]);
    for e in &state.borrow().epsilon {
        add_state(&e, states);
    }
}

impl NFA {
    pub fn new(start: SState, end: SState) -> NFA {
        end.borrow_mut().is_end = true;
        NFA { start, end }
    }

    pub fn match_regex(&mut self, to_match: &str) -> bool {
        let mut current_states = Vec::<SState>::new();
        add_state(&self.start, &mut current_states);

        for c in to_match.chars() {
            let mut next_states = Vec::<SState>::new();
            for state in current_states {
                if state.borrow().transitions.contains_key(&c) {
                    let x = state.borrow();
                    let trans_state = x.transitions.get(&c).unwrap();
                    add_state(trans_state, &mut next_states);
                }
            }
            current_states = next_states;
        }

        for s in current_states {
            if s.borrow().is_end {
                return true;
            }
        }
        false
    }
}

// ----------------------------------

pub struct Handler {
    state_count: usize,
}

impl Handler {
    fn create_state(&mut self) -> Rc<RefCell<State>> {
        self.state_count += 1;
        Rc::new(RefCell::new(State::new(format!("s{}", self.state_count))))
    }

    fn handle_char(
        &mut self,
        t: &Token,
        nfa_stack: &mut Vec<NFA>,
    ) -> Result<(), NfaConstructionError> {
        let v: char = if let Token::Char(v) = *t {
            v
        } else {
            return Err(NfaConstructionError::new(format!(
                "expecting Token::Char, got={}",
                *t
            )));
        };

        let s0 = self.create_state();
        let s1 = self.create_state();
        s0.borrow_mut()
            .transitions
            .entry(v)
            .or_insert_with(|| Rc::clone(&s1));

        let nfa = NFA::new(s0, s1);
        let mut nfa = vec![nfa];
        nfa_stack.append(&mut nfa);
        Ok(())
    }

    fn handle_concat(&mut self, _t: &Token, nfa_stack: &mut Vec<NFA>) {
        let n2 = nfa_stack.pop().unwrap();
        let n1 = nfa_stack.pop().unwrap();
        n1.end.borrow_mut().is_end = false;
        n1.end
            .borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&n2.start)]);
        let nfa = NFA::new(n1.start, n2.end);
        nfa_stack.append(&mut vec![nfa]);
    }

    fn handle_alt(&mut self, _t: &Token, nfa_stack: &mut Vec<NFA>) {
        let n2 = nfa_stack.pop().unwrap();
        let n1 = nfa_stack.pop().unwrap();
        let s0 = self.create_state();
        s0.borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&n1.start), Rc::clone(&n2.start)]);
        let s3 = self.create_state();
        n1.end
            .borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&s3)]);
        n2.end
            .borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&s3)]);
        n1.end.borrow_mut().is_end = false;
        n2.end.borrow_mut().is_end = false;
        let nfa = NFA::new(s0, s3);
        nfa_stack.append(&mut vec![nfa]);
    }

    fn handle_rep(&mut self, t: &Token, nfa_stack: &mut Vec<NFA>) {
        let n1 = nfa_stack.pop().unwrap();
        let s0 = self.create_state();
        let s1 = self.create_state();

        s0.borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&n1.start)]);

        if let Token::Star = t {
            s0.borrow_mut().epsilon.append(&mut vec![Rc::clone(&s1)]);
        }

        n1.end
            .borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&s1), Rc::clone(&n1.start)]);
        n1.end.borrow_mut().is_end = true;

        let nfa = NFA::new(s0, s1);
        nfa_stack.append(&mut vec![nfa]);
    }

    fn handle_qmark(&mut self, _t: &Token, nfa_stack: &mut Vec<NFA>) {
        let n1 = nfa_stack.pop().unwrap();
        n1.start
            .borrow_mut()
            .epsilon
            .append(&mut vec![Rc::clone(&n1.end)]);
        nfa_stack.append(&mut vec![n1]);
    }

    pub fn handle(
        &mut self,
        t: &Token,
        nfa_stack: &mut Vec<NFA>,
    ) -> Result<(), NfaConstructionError> {
        match t {
            Token::Star => self.handle_rep(t, nfa_stack),
            Token::Alt => self.handle_alt(t, nfa_stack),
            Token::Concat => self.handle_concat(t, nfa_stack),
            Token::Plus => self.handle_rep(t, nfa_stack),
            Token::QMark => self.handle_qmark(t, nfa_stack),
            Token::Char(_) => return self.handle_char(t, nfa_stack),
            _ => {
                return Err(NfaConstructionError::new(format!(
                    "not expecting this token type: {}",
                    t
                )))
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        Handler { state_count: 0 }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Handler::new()
    }
}

// ----------------------------------

pub fn compile(pattern: String) -> Result<NFA, Box<dyn Error>> {
    let lexer = Lexer::new(pattern);
    let mut parser = Parser::new(lexer);
    let tokens = parser.parse(true)?;

    let mut handler = Handler::new();

    let mut nfa_stack = vec![];
    for t in tokens {
        handler.handle(t, &mut nfa_stack)?;
    }

    assert_eq!(nfa_stack.len(), 1);
    Ok(nfa_stack.pop().unwrap())
}
