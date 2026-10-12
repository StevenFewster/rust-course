use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn main() {
    let vars: HashMap<String, f64> = HashMap::new();
    let stdin = io::stdin();

    print!("> ");
    io::stdout().flush().ok();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        // TODO(milestone 5): tokenize, detect `let name = …`, parse, eval, print result or error.
        let _ = (&line, &vars);
        println!("(not implemented yet)");
        print!("> ");
        io::stdout().flush().ok();
    }
}
