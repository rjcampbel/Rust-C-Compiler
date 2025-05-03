mod cli;
mod lexer;
mod preprocessor;

use cli::Cli;
use lexer::Lexer;
use preprocessor::Preprocessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();
    let pp_contents = Preprocessor::new(args.file).process()?;
    let tokens = Lexer::new(pp_contents).lex()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

