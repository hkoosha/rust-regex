/// Implementation inspired from:
/// https://github.com/deniskyashif/regexjs
///
use crate::regex1::nfa::{NFA, postfix_to_nfa, recognize};
use crate::regex1::parser::{to_postfix, with_explicit_concat};

pub mod nfa;
pub mod parser;

pub fn create_matcher(exp: &str) -> Result<NFA, String> {
    let explicit = with_explicit_concat(exp);
    let postfix = to_postfix(&explicit);
    postfix_to_nfa(&postfix)
}

pub fn main0() -> Result<(), String> {
    let exp = "ab*c(d?w)*|w";
    let implicit = with_explicit_concat(exp);
    println!("{}", implicit);

    let postfix = to_postfix(&implicit);
    println!("{}", postfix);

    let nfa = create_matcher(exp)?;

    println!("{}", recognize(&nfa, "abc"));
    println!("{}", recognize(&nfa, "dac"));

    Ok(())
}
