mod cli;

use cli::Cli;
use std::process;
use std::fs;

fn main() {
    let args: Cli = Cli::do_parse();

    println!("Args {:?}", args);

    let contents = fs::read_to_string(&args.file).unwrap_or_else(|err|{
        println!("Failed to open \"{0}\": {err}", args.file);
        process::exit(1);
    });

    let mut chars = contents.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => println!("found an open paren"),
            ')' => println!("found a close paren"),
            ';' => println!("found a semicolon"),
            '{' => println!("found an open brace"),
            '}' => println!("found a close brace"),
            _ if c.is_alphabetic() => println!("found a character: {c}"),
            _ if c.is_whitespace() => println!("found whitespace"),
            _ if c.is_ascii_digit() => println!("found a digit: {c}"),
            _ => println!("invalid character")
        }
    }
}
