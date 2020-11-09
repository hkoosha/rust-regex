use regex::regex1::create_matcher;
use regex::regex1::nfa::recognize;
use regex::regex1::parser::{to_postfix, with_explicit_concat};

fn main() -> Result<(), String> {
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
