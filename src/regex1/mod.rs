/// Implementation inspired from:
/// https://github.com/deniskyashif/regexjs
///
use crate::regex1::nfa::{postfix_to_nfa, NFA};
use crate::regex1::parser::{to_postfix, with_explicit_concat};

pub mod nfa;
pub mod parser;
pub mod parser2;

pub fn create_matcher(exp: &str) -> Result<NFA, String> {
    let explicit = with_explicit_concat(exp);
    let postfix = to_postfix(&explicit);
    postfix_to_nfa(&postfix)
}
