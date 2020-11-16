use regex::Regex;
use std::env::args;
use std::io::{BufReader, BufRead, Write};

const DO_LOOP: bool = true;

fn main() {
    logwatch(None);
}

// arg_override: let the test code provide env args
fn logwatch(arg_override: Option<(String,String)>) {
    let file;
    let pattern;
    // Allow test code to override env args
    if arg_override.is_some() {
        let arg_override = arg_override.unwrap();
        file = arg_override.0;
        pattern = arg_override.1;
    } else {
        let mut args = args();
        file = match args.nth(1) {
            Some(f) => f,
            None => {
                println!("Usage: logwatch <FILE> <PATTERN> (No file)");
                return;
            }
        };
        pattern = match args.next() {
            Some(f) => f,
            None => {
                println!("Usage: logwatch <FILE> <PATTERN> (No pattern)");
                return;
            }
        };
    }

    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(err) => {
            println!("Invalid Regex {}", err);
            return
        }
    };

    let file = match std::fs::File::open(&file) {
        Ok(f) => f,
        Err(err) => {
            println!("Failed to open file \"{}\"", file);
            println!("{}", err);
            return;
        }
    };

    // Best practice: buffer reading
    let mut read = BufReader::new(file);
    // Create a buffer for the read lines
    let mut buf = String::new();
    // Until interrupted:
    loop {
        // Clear the line-buffer
        buf.clear();
        // Read until the next newline
        let bytes_read = match read.read_line(&mut buf) {
            Ok(n) => n,
            Err(err) => {
                println!("Failed to read from file: {:?}", err);
                return
            }
        };
        // If we read nothing, sleep for a bit, then try again
        if bytes_read == 0 {
            // If 'DO_LOOP' is true and we just read 0 bytes, wait 100ms and try to read more lines
            // Otherwise, exit the program
            if DO_LOOP {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            } else {
                break;
            }
        }
        // If we did read something, check if it matches the pattern
        if reg.is_match(&buf) {
            print!("{}", &buf);
            // stdout might be buffered. To make sure we see results immediately, flush it
            match std::io::stdout().flush() {
                Ok(_) => (),
                Err(err) => eprintln!("Failed to flush stdout: {}", err),
            }
        }
    }
}

#[cfg(test)]
mod test;