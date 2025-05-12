mod assembler;
mod assembly;
mod cli;
mod lexer;
mod preprocessor;
mod parser;
mod tacky;

use cli::Cli;
use assembler::Assembler;
use assembly::AssemblyGen;
use assembly::at;
use lexer::Lexer;
use parser::Parser;
use parser::ast::Program;
use preprocessor::Preprocessor;
use std::error::Error;
use std::fs::File;
use tacky::tacky_ast;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::do_parse();

    let pp_contents = Preprocessor::new(&args.file).process()?;

    if args.command.run_lexer {
        let tokens = Lexer::new(pp_contents).lex()?;
        for token in &tokens {
            println!("{:?}", token);
        }

        let program: Program;
        if args.command.run_parser {
            program = Parser::new(tokens).parse()?;
            program.pretty_print();

            if args.command.run_tacky {
                let tacky_program = tacky_ast::Program::parse(&program)?;
                tacky_program.pretty_print();
            }
            // if args.command.run_codegen {
            //     let at_program: at::Program;
            //     at_program = AssemblyGen::new(program).parse()?;
            //     at_program.pretty_print();

            //     let path: String = args.file.replace(".c", ".s");
            //     let mut code = File::create(&path)?;
            //     at_program.write(&mut code)?;

            //     if args.command.run_assembler {
            //         Assembler::new(&path).process()?;
            //     }
            // }
        }
    }

    Ok(())
}

