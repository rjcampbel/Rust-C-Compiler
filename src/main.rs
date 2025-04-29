mod cli;
mod identifier;

use cli::Cli;
use std::process;
use std::fs;

fn main() {
    let args: Cli = Cli::do_parse();

    println!("Args {:?}", args);

    let contents = fs::read_to_string(&args.file).unwrap_or_else(|err|{
        println!("Failed to open \"{0}\": {err}", args.file);
        process::exit(1);
    });

    println!("{contents}");
}
