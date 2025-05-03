mod cli;
mod ast;


use cli::Cli;
use std::process;
use std::process::Command;
use std::fs;

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Int,
    Void,
    Return,
    Integer(u64),
    Identifier(String),
}

fn main() {
    let args: Cli = Cli::do_parse();

    println!("Args {:?}", args);

    let output_filename = args.file.replace(".c", "_pp.c");
    let _ = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(&args.file)
        .arg("-o")
        .arg(&output_filename)
        .output();

    let contents = fs::read_to_string(&output_filename).unwrap_or_else(|err|{
        println!("Failed to open \"{0}\": {err}", args.file);
        process::exit(1);
    });

    let mut tokens: Vec<Token> = Vec::new();
    let mut error: bool = false;
    let mut chars = contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ';' => tokens.push(Token::Semicolon),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            _ if c.is_alphabetic() || c == '_' => {
                let mut token_value: String = String::from(c);
                while let Some(t) = chars.peek() {
                    if t.is_alphanumeric() || *t == '_' {
                        token_value.push(chars.next().unwrap());
                    } else {
                        match token_value.as_str() {
                            "int" => tokens.push(Token::Int),
                            "void" => tokens.push(Token::Void),
                            "return" => tokens.push(Token::Return),
                            _ => tokens.push(Token::Identifier(token_value))
                        }
                        break;
                    }
                }
            },
            _ if c.is_ascii_digit() => {
                let mut token_value: String = String::from(c);
                while let Some(t) = chars.peek() {
                    if t.is_ascii_digit() {
                        token_value.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(t) = chars.peek() {
                    if t.is_alphabetic() {
                        error = true;
                    }
                } else {
                    tokens.push(Token::Integer(token_value.parse::<u64>().unwrap()));
                }
            },
            _ if c.is_whitespace() => (),
            _ => {
                error = true;
                println!("invalid token");
            }
        }
    }

    for token in tokens {
        println!("{:?}", token);
    }

    let _ = std::fs::remove_file(&output_filename);

    if error {
        std::process::exit(1);
    }
}
