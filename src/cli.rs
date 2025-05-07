use clap::{
    Args,
    builder::ArgPredicate,
    Parser
};

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Source file to compile
    pub file: String,

    #[command(flatten)]
    pub command: Command,
}

impl Cli {
   pub fn do_parse() -> Cli {
      Cli::parse()
   }
}

#[derive(Args,Debug)]
#[group(required = false, multiple = false)]
pub struct Command
{
    /// Run the lexer, parser, codegen, and assembler
    #[arg(id= "assemble", short, long, default_value_t = true, default_value_ifs([("lex", ArgPredicate::IsPresent, Some("false")), ("parse", ArgPredicate::IsPresent, Some("false")), ("codegen", ArgPredicate::IsPresent, Some("false"))]))]
    pub run_assembler: bool,

    /// Run the the lexer, parser, and codegen
    #[arg(id = "codegen", short, long, default_value_t = true, default_value_ifs([("lex", ArgPredicate::IsPresent, Some("false")), ("parse", ArgPredicate::IsPresent, Some("false"))]))]
    pub run_codegen: bool,

    /// Run the lexer and the parser
    #[arg(id = "parse", short, long, default_value_t = true, default_value_if("lex", ArgPredicate::IsPresent, Some("false")))]
    pub run_parser: bool,

    /// Run the lexer
    #[arg(id = "lex", short, long, default_value_t = true, conflicts_with_all(["parse", "codegen"]))]
    pub run_lexer: bool,
}