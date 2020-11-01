use regex::regex1::parser::{with_explicit_concat, to_postfix};
use regex::regex1::create_matcher;
use std::borrow::Borrow;

fn main() -> Result<(), String> {
    let exp = "ab*c(d?w)*|w";
    let implicit = with_explicit_concat(exp);
    println!("{}", implicit);

    let postfix = to_postfix(&implicit);
    println!("{}", postfix);

    let nfa = create_matcher(exp)?;

    Ok(())
}
