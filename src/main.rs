mod cli;
mod lexer;
mod preprocessor;

use cli::Cli;
use preprocessor::Preprocessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::do_parse();

    println!("Args {:?}", args);

    let pp_contents = Preprocessor::new(args.file).process()?;

    let tokens = lexer::parse(pp_contents)?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

