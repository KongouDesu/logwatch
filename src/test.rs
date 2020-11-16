#[cfg(test)]
mod test {
    use crate::logwatch;
    use std::io::Write;
    use rand;
    use rand::Rng;
    use std::time::Duration;

    #[test]
    // Spawn a thread to write to a test logfile,
    fn continuous() {
        if !crate::DO_LOOP {
            panic!("DO_LOOP must be true for this test work correctly");
        }

        // Create test.log with some initial data
        std::fs::write("test.log",
                       r"[package]
edition
Word
sentence with no words starting with a capital letter
this sentence HAS a word that'll match
6375
£645@£€5
").expect("Failed to create test.log");

        // Start CLI on test.log, returning every line that has a word starting with a capital letter
        std::thread::spawn(|| logwatch(Some(("test.log".to_string(),r"([A-Z])\w+".to_string()))));

        // Open the test.log file in append mode
        let mut log = std::fs::OpenOptions::new().append(true).open("test.log")
            .expect("Failed to open test.log for writing");

        // Every 250ms, append one of these lines (chosen randomly)
        let lines = vec!["Critical error","2020-04-23 14:24:53","¤%/%¤/%&(!(&/€${","foo","bar","baz","Alice","Bob","lorem ipsum"];
        let mut rand = rand::thread_rng();
        for _ in 0..50 {
            log.write_all(lines[rand.gen_range(0,lines.len())].as_bytes()).expect("Failed to write to file");
            log.write_all(b"\n").expect("Failed to write to file");
            std::thread::sleep(Duration::from_millis(250));
        }
        // We should expect see 'Word' and 'this sentence HAS a word that'll match' from the initial data
        // Followed by a random sequence of only 'Critical Error', 'Alice' and 'Bob'
    }
}