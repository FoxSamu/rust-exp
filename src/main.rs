mod expression;
mod parser;

use std::io::{stdin, stdout, Write};

use crate::parser::*;

fn main() {
    let mut ln = String::new();

    // Infinite loop
    loop {
        // Clear line
        ln.clear();

        // Write >>>, then flush so it appears in the terminal
        print!(">>> ");
        stdout().flush().expect("Failed to flush stdout");

        // Read input line
        stdin().read_line(&mut ln).expect("Failed to read input");

        // Parse input line, let parser borrow our string
        match parse(&ln) {
            // Syntax error, print error
            ParseResult::Error(x, i) => {
                println!("!!! {}, at index {}", x, i)
            },

            // Empty input, exit
            ParseResult::Absent => {
                println!("Goodbye");
                break;
            },

            // Successful parse, evaluate and print
            ParseResult::Present(exp) => {
                println!("<<< {}", exp.eval())
            }
        }
    }
}
