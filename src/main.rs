mod assembler;
mod assembly;
mod cli;
mod lexer;
mod parser;
mod preprocessor;
mod tacky;

use assembler::Assembler;
use assembly::AssemblyGen;
use assembly::at;
use cli::Cli;
use lexer::Lexer;
use parser::Parser;
use preprocessor::Preprocessor;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();

    if args.command.run_lexer {
        let pp_contents = Preprocessor::new(&args.file).process()?;
        let tokens = Lexer::new(pp_contents).lex()?;
        for token in &tokens {
            println!("{:?}", token);
        }

        if args.command.run_parser {
            let program = Parser::new(tokens).parse()?;
            program.pretty_print();

            if args.command.run_tacky {
                let tacky_program = tacky::Tacky::new(program).generate()?;
                tacky_program.pretty_print();

                if args.command.run_codegen {
                    let at_program: at::Program;
                    at_program = AssemblyGen::new(tacky_program).parse()?;
                    at_program.pretty_print();

                    if args.command.run_assembler {
                        let path: String = args.file.replace(".c", ".s");
                        let mut code = File::create(&path)?;
                        at_program.write(&mut code)?;
                        Assembler::new(&path).process()?;
                    }
                }
            }
        }
    }

    Ok(())
}

