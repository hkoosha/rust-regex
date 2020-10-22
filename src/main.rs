use regex::compile;

fn main() {
    let mut nfa = compile("ab*".to_string());

    assert!(nfa.match_regex(&"ab"));
    assert!(nfa.match_regex(&"abbbb"));
    assert!(nfa.match_regex(&"a"));
    assert!(!nfa.match_regex(&"baaaab"));
}
