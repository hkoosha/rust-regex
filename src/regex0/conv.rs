use std::collections::HashMap;

const OPS: [char; 5] = ['|', '?', '+', '*', '^'];
const BIN_OPS: [char; 2] = ['^', '|'];

fn op_precedence() -> HashMap<char, usize> {
    let precedence: HashMap<_, _> = vec![
        ('(', 1 as usize),
        ('|', 2),
        ('.', 3),
        ('?', 4),
        ('*', 4),
        ('+', 4),
        ('^', 5),
    ]
    .into_iter()
    .collect();

    precedence
}

fn format_regex(regex: String) -> String {
    let mut formatted = String::with_capacity(regex.len());
    for (i, c1) in regex.as_str().chars().enumerate() {
        if i + 1 >= regex.len() {
            continue;
        }
        let c2 = regex.chars().nth(i + 1).unwrap();

        formatted.push(c1);
        if c1 != '(' && c2 != ')' && !OPS.contains(&c2) && !BIN_OPS.contains(&c1) {
            formatted.push('.');
        }
    }
    formatted.push(regex.chars().nth(regex.len() - 1).unwrap());
    formatted
}

pub fn infix_to_postfix(regex: String) -> String {
    let precedence = op_precedence();
    let lowest_precedence = precedence.iter().map(|x| x.1).max().unwrap() + 1;

    let regex = format_regex(regex);
    eprintln!("{}", regex);

    let mut postfix = String::with_capacity(regex.len());
    let mut stack = Vec::with_capacity(postfix.len());

    for c in regex.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                while *stack.last().unwrap() != '(' {
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            _ => {
                while !stack.is_empty() {
                    let l = stack.last().unwrap();
                    let l_precedence = precedence.get(&l).unwrap_or(&lowest_precedence);
                    let c_precedence = precedence.get(&c).unwrap_or(&lowest_precedence);

                    if l_precedence >= c_precedence {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
        }
    }

    while let Some(popped) = stack.pop() {
        postfix.push(popped);
    }

    postfix
}
