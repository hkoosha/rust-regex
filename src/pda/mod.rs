use std::collections::HashMap;

struct PDA {
    start_input: String,
    found: usize,
    accepted_config: Vec<usize>,
    production_rules: HashMap<char, char>,
    states: Vec<usize>,
    symbols: Vec<usize>,
    stack_symbols: Vec<usize>,
    start_symbol: String,
    stack_start: String,
    acceptable_states: Vec<usize>,
    accept_with: String,
}

impl PDA {
    fn generate(&self) -> usize {
        let total = 0usize;

        if self.found > 0 {
            return 0;
        } else {
            return 1;
        }
    }

    fn is_found(&self, state: usize, input: &str, stack: usize) -> bool {
        if input.len() > 0 {
            return false;
        }
        todo!()
        // if self.accept_with == "E" {
        //     return self.stack
        // }
    }
}
