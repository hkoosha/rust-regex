# What is this?

This is just a toy. Me playing around with rust. Nothing serious. If you're looking 
for proper fast regex handling, see the regex crate.
A good read: https://swtch.com/~rsc/regexp (regex crate complies with what Russ Cox says).

# rust-regex
Regular expression engine in Rust using Thompson's algorithm

One of implementations inspired by: https://github.com/xysun/regex

The other implementation inspired by: https://github.com/deniskyashif/regexjs


# Memory Leak!
This implementation freely uses `Rc` strong clones, not the weak clones, and also 
contains a graph. Hence there's a memory leak!
