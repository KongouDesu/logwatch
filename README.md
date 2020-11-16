# logwatch

# Usage
`logwatch <FILE> <PATTERN>`

# Args
`<FILE>`: Path to a file, relative or absolute  
`<PATTERN>`: A valid RegEx pattern<sup>1</sup>

# Running
Built for the latest stable version of Rust (1.47.0)  
Build with `cargo build` then run executable in `target/debug/` **OR** `cargo run <FILE> <PATTERN>`

# Loop vs. no loop
Supports 2 modes, either read and match on the entire file once, then exit, or keep the file open, printing any newly-added lines that matches the pattern.

To change this behavior, switch `DO_LOOP` to true/false at the top of `src/main.rs`

To showcase the looping mode, set `DO_LOOP = true`, then run `cargo test`




<sup>1: Uses the Rust [regex](https://docs.rs/regex/1.4.2/regex/) crate, which (deliberately) does not support backreferencing and lookahead</sup>
