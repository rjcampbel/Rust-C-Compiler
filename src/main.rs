mod assembly;
mod cli;
mod lexer;
mod preprocessor;
mod parser;

use cli::Cli;
use assembly::AssemblyGen;
use assembly::at;
use lexer::Lexer;
use parser::Parser;
use parser::ast::Program;
use preprocessor::Preprocessor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();

    let pp_contents = Preprocessor::new(args.file).process()?;

    if args.command.run_lexer {
        let tokens = Lexer::new(pp_contents).lex()?;
        for token in &tokens {
            println!("{:?}", token);
        }

        let program: Program;
        if args.command.run_parser {
            program = Parser::new(tokens).parse()?;
            program.pretty_print();

            let at_program: at::Program;
            if args.command.run_codegen {
                at_program = AssemblyGen::new(program).parse()?;
                at_program.pretty_print();
            }
        }
    }

    Ok(())
}

