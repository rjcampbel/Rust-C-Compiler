mod cli;

use cli::Cli;
use std::process;
use std::fs;

use nom::{
    IResult,
    Parser,
    branch::alt,
    character::complete::{alpha1,alphanumeric1},
    bytes::complete::tag,
    multi::many0_count,
    sequence::pair,
    combinator::recognize
  };


pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_"))))
        )
    ).parse(input)
}

fn main() {
    let args: Cli = Cli::do_parse();

    println!("Args {:?}", args);

    let contents = fs::read_to_string(&args.file).unwrap_or_else(|err|{
        println!("Failed to open \"{0}\": {err}", args.file);
        process::exit(1);
    });

    println!("{contents}");
}

#[cfg(test)]
mod tests {
    use crate::identifier;
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    #[test]
    fn first_test() {
        assert_eq!(identifier("_input"), Ok(("", "_input")));
        assert_eq!(identifier("hello_input"), Ok(("", "hello_input")));
        assert_eq!(identifier("123_input"), Err(Err::Error(Error::new("123_input", ErrorKind::Tag))));
        assert_eq!(identifier("input123___foobar"), Ok(("", "input123___foobar")));
        assert_eq!(identifier("___"), Ok(("", "___")));
        assert_eq!(identifier("__;f647"), Ok((";f647", "__")));
    }
}