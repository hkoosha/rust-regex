/// Implementation inspired from:
/// https://gist.github.com/gmenard/6161825
///
use crate::regex0::conv::infix_to_postfix;
use crate::regex0::regex::compile;

pub mod constructs;
pub mod conv;
pub mod functionals;
pub mod regex;

pub fn main0() {
    let mut nfa = compile("ab*".to_string()).expect("error parsing regex");

    println!("matching ab");
    assert!(nfa.match_regex(&"ab"));

    println!("matching abbbb");
    assert!(nfa.match_regex(&"abbbb"));

    println!("matching a");
    assert!(nfa.match_regex(&"a"));

    println!("matching baaaab");
    assert!(!nfa.match_regex(&"baaaab"));
}

pub fn main1() {
    let string = std::env::args()
        .nth(1)
        .expect("expecting one argument (the regex)");
    println!("{}", infix_to_postfix(string));
}
