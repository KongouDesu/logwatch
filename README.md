# logwatch

# Usage
`logwatch <FILE> <PATTERN>`

# Args
`<FILE>`: Path to a file, relative or absolute  
`<PATTERN>`: A valid RegEx pattern<sup>1</sup>

# Running
Built for the latest stable version of Rust (1.47.0)  
Build with `cargo build` then run executable in `target/debug/` **OR** `cargo run <FILE> <PATTERN>`  

An example pattern could be `([A-Z])\w` which will match any line that contains a word starting with an uppercase letter

# Loop vs. no loop
Supports 2 modes, either read and match on the entire file once, then exit, or keep the file open, printing any newly-added lines that matches the pattern.

To change this behavior, switch `DO_LOOP` to true/false at the top of `src/main.rs`

To showcase the looping mode, set `DO_LOOP = true`, then run `cargo test`

# Further details
The program is written in Rust, using only 1 external library, the `regex`<sup>1</sup> crate , used to handle the pattern matching.  

The code will interpret the first 2 arguments passed as the file and pattern respectively. It will verify the pattern is a valid RegEx, then try to open the file. 
If the file exists and we have permission to read it, the program will proceed to read each line, check if it matches the RegEx and if it does, print out the line.

If `DO_LOOP` is enabled, the program will keep waiting when it reaches the end of the file. Whenever more data is written, it will read it and check it against the RegEx. If `DO_LOOP` is disabled, the program will simply exit when reaching the end of the file.

# Caveats
The program requires the logfile to be valid UTF-8. This is not inherently an issue, as we are searching text log files.  

It is assumed that, when looping, new data is only appended. If the file is written in out-of-order or truncated, the behaviour is undefined.
Typically logs are only appended to, so this shouldn't be a practical concern.

<sup>1: Uses the [regex](https://docs.rs/regex/1.4.2/regex/) crate (library), which (deliberately) does not support backreferencing and lookahead</sup>
