mod cli;
mod lexer;
mod preprocessor;
mod parser;

use cli::Cli;
use lexer::Lexer;
use parser::Parser;
use preprocessor::Preprocessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();
    let pp_contents = Preprocessor::new(args.file).process()?;
    let tokens = Lexer::new(pp_contents).lex()?;

    for token in &tokens {
        println!("{:?}", token);
    }

    let program = Parser::new(tokens).parse()?;
    program.pretty_print();

    Ok(())
}

