use std::collections::HashMap;

pub fn with_explicit_concat(str: &str) -> String {
    let mut output = String::with_capacity(str.len());

    for (i, token) in str.chars().enumerate() {
        output.push(token);

        if token != '(' && token != '|' && i < str.len() - 1 {
            match str.chars().nth(i + 1).unwrap() {
                '*' | '?' | '+' | '|' | ')' => {}
                _ => output.push('.'),
            }
        }
    }

    output
}

pub fn to_postfix(str: &str) -> String {
    let precedence: HashMap<char, usize> = vec![('|', 0), ('.', 1), ('?', 2), ('*', 2), ('+', 2)]
        .into_iter()
        .collect();
    let mut output = String::with_capacity(str.len());
    let mut operator_stack: Vec<char> = vec![];

    for token in str.chars() {
        match token {
            '.' | '|' | '*' | '?' | '+' => {
                while !operator_stack.is_empty()
                    && *operator_stack.last().unwrap() != '('
                    && precedence[operator_stack.last().unwrap()] >= precedence[&token]
                {
                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(token);
            }
            '(' => {
                operator_stack.push('(');
            }
            ')' => {
                while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '(' {
                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop();
            }
            _ => {
                output.push(token);
            }
        }
    }

    while !operator_stack.is_empty() {
        output.push(operator_stack.pop().unwrap());
    }

    output
}
