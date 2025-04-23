use clap::{builder::ArgPredicate, Args, Parser};
use std::process;
use std::fs;

/// Simple program to greet a person
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Source file to compile
    file: String,

    #[command(flatten)]
    command: Command,
}

#[derive(Args,Debug)]
#[group(required = false, multiple = false)]
struct Command
{
    /// Run the the lexer, parser, and codegen
    #[arg(id = "codegen", short, long, default_value_t = true, default_value_ifs([("lex", ArgPredicate::IsPresent, Some("false")), ("parse", ArgPredicate::IsPresent, Some("false"))]))]
    run_codegen: bool,

    /// Run the lexer and the parser
    #[arg(id = "parse", short, long, default_value_t = true, default_value_if("lex", ArgPredicate::IsPresent, Some("false")))]
    run_parser: bool,

    /// Run the lexer
    #[arg(id = "lex", short, long, default_value_t = true, conflicts_with_all(["parse", "codegen"]))]
    run_lexer: bool,
}

fn main() {
    let args: Cli = Cli::parse();

    println!("Args {:?}", args);

    let contents = fs::read_to_string(&args.file).unwrap_or_else(|err|{
        println!("Failed to open \"{0}\": {err}", args.file);
        process::exit(1);
    });

    println!("{contents}");
}
