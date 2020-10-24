use regex::compile;

fn main() {
    let mut nfa = compile("ab*".to_string()).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    println!("matching ab");
    assert!(nfa.match_regex(&"ab"));

    println!("matching abbbb");
    assert!(nfa.match_regex(&"abbbb"));

    println!("matching a");
    assert!(nfa.match_regex(&"a"));

    println!("matching baaaab");
    assert!(!nfa.match_regex(&"baaaab"));
}
