use std::collections::HashMap;

pub fn with_explicit_concat(str: &str) -> String {
    let mut output = String::with_capacity((str.len() as f32 * 1.5) as usize);

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

//  Recursive descent parser for regular expressions. Implements the following grammar:
//
//  Expr -> Term | Term '|' Expr
//  Term -> Factor | Factor Term
//  Factor -> Atom | Atom MetaChar
//  Atom -> Char | '(' Expr ')'
//  Char -> AnyCharExceptMeta | '\' AnyChar
//  MetaChar -> '?' | '*' | '+'
//
//

pub struct TreeNode {
    pub label: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(label: String, children: Vec<TreeNode>) -> TreeNode {
        TreeNode { label, children }
    }

    fn from_label(label: char) -> TreeNode {
        TreeNode {
            label: String::from(label),
            children: vec![],
        }
    }
}

const fn is_meta_char(c: char) -> bool {
    matches!(c, '*' | '+' | '?')
}

#[derive(Debug)]
pub struct Parser {
    pattern: String,
    position: usize,
    parsed: bool,
}

impl Parser {
    pub fn new(pattern: String) -> Parser {
        Parser {
            pattern,
            position: 0,
            parsed: false,
        }
    }

    pub fn parse(&mut self) -> Result<TreeNode, String> {
        if self.parsed {
            return Err("already parsed".to_string());
        }

        self.parsed = true;
        self.expression()
    }

    fn expression(&mut self) -> Result<TreeNode, String> {
        let term = self.term()?;

        let children = if self.has_more_chars() && self.peek() == '|' {
            self.match_('|')?;
            let expr = self.expression()?;
            vec![term, TreeNode::from_label('|'), expr]
        } else {
            vec![term]
        };

        Ok(TreeNode::new("Expr".to_string(), children))
    }

    fn term(&mut self) -> Result<TreeNode, String> {
        let factor = self.factor()?;

        let children = if self.has_more_chars() && self.peek() != ')' && self.peek() != '|' {
            vec![factor, self.term()?]
        } else {
            vec![factor]
        };

        Ok(TreeNode::new("Term".to_string(), children))
    }

    fn factor(&mut self) -> Result<TreeNode, String> {
        let atom = self.atom()?;

        let children = if self.has_more_chars() && is_meta_char(self.peek()) {
            vec![atom, TreeNode::from_label(self.next()?)]
        } else {
            vec![atom]
        };

        Ok(TreeNode::new("Factor".to_string(), children))
    }

    fn atom(&mut self) -> Result<TreeNode, String> {
        let children = if self.peek() == '(' {
            self.match_('(')?;
            let expr = self.expression()?;
            self.match_(')')?;
            vec![TreeNode::from_label('('), expr, TreeNode::from_label(')')]
        } else {
            vec![self.char_()?]
        };
        Ok(TreeNode::new("Atom".to_string(), children))
    }

    fn char_(&mut self) -> Result<TreeNode, String> {
        if is_meta_char(self.peek()) {
            return Err(format!("unexpected meta char={}", self.peek()));
        }

        let children = if self.peek() == '\\' {
            self.match_('\\')?;
            vec![
                TreeNode::from_label('\\'),
                TreeNode::from_label(self.next()?),
            ]
        } else {
            vec![TreeNode::from_label(self.next()?)]
        };

        Ok(TreeNode::new("Char".to_string(), children))
    }

    // --------------

    fn next(&mut self) -> Result<char, String> {
        let c = self.peek();
        self.match_(c)?;
        Ok(c)
    }

    fn match_(&mut self, symbol: char) -> Result<(), String> {
        if self.peek() != symbol {
            Err(format!("unexpected symbol: {}", symbol))
        } else {
            self.position += 1;
            Ok(())
        }
    }

    fn has_more_chars(&self) -> bool {
        self.position < self.pattern.len()
    }

    fn peek(&self) -> char {
        self.pattern.chars().nth(self.position).unwrap()
    }
}
