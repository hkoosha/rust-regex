# rust-regex
Regular expression engine in Rust using Thompson's algorithm (port of xysun's implementation)

Original implementation in Python: https://github.com/xysun/regex


This implementation freely uses `Rc` strong clones, not the weak clones, and also contains a graph.
Hence there's a memory leak!
