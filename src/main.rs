mod cli;
mod lexer;
mod preprocessor;
mod parser;

use cli::Cli;
use lexer::Lexer;
use lexer::token::Token;
use parser::Parser;
use parser::ast::Program;
use preprocessor::Preprocessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();

    let pp_contents = Preprocessor::new(args.file).process()?;

    let mut tokens: Vec<Token> = Vec::new();
    if args.command.run_lexer {
        tokens = Lexer::new(pp_contents).lex()?;
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    let program: Program;
    if args.command.run_parser {
        program = Parser::new(tokens).parse()?;
        program.pretty_print();
    }

    Ok(())
}

