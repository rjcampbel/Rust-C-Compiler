use nom::{
   Parser,
   IResult,
   combinator::map,
   combinator::recognize,
   multi::many0,
   sequence::pair,
   character::complete::{alpha1,alphanumeric1},
   branch::alt,
   bytes::complete::tag,
};

#[derive(Eq, PartialEq, Debug)]
pub struct IdentifierToken {
   pub name: String
}

pub fn parse_identifier_first_char(input: &str) -> IResult<&str, &str> {
   alt((alpha1, tag("_"))).parse(input)
}

pub fn parse_identifier_remaining_chars(input: &str) -> IResult<&str, &str> {
   recognize(many0(alt((alphanumeric1, tag("_"))))).parse(input)
}

impl IdentifierToken {
   fn parse(input: &str) -> IResult<&str, Self> {
      let parse_identifier = recognize(pair(parse_identifier_first_char, parse_identifier_remaining_chars));

      map(parse_identifier, |s: &str| IdentifierToken { name: String::from(s) }).parse(input)
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use nom::error::{Error, ErrorKind};
   use nom::Err;

   #[test]
   fn test_identifiers() {
      let passing_test_cases = [
         ("_", IdentifierToken { name: "_".to_owned() }, ""),
         ("__", IdentifierToken { name: "__".to_owned() }, ""),
         ("_123", IdentifierToken { name: "_123".to_owned() }, ""),
         ("_abc", IdentifierToken { name: "_abc".to_owned() }, ""),
         ("abc", IdentifierToken { name: "abc".to_owned() }, ""),
         ("abc123", IdentifierToken { name: "abc123".to_owned() }, ""),
         ("abc_123", IdentifierToken { name: "abc_123".to_owned() }, ""),
         ("abc 123", IdentifierToken { name: "abc".to_owned() }, " 123"),
         ("abc(", IdentifierToken { name: "abc".to_owned() }, "("),
         ("abc{", IdentifierToken { name: "abc".to_owned() }, "{"),
         ("abc)", IdentifierToken { name: "abc".to_owned() }, ")"),
         ("abc}", IdentifierToken { name: "abc".to_owned() }, "}"),
         ("abc;", IdentifierToken { name: "abc".to_owned() }, ";"),
         ("_abc_;", IdentifierToken { name: "_abc_".to_owned() }, ";"),
      ];

      for (input, expected_output, expected_remaining_input) in passing_test_cases {
         let (remaining_input, output) = IdentifierToken::parse(input).unwrap();
         assert_eq!(output, expected_output);
         assert_eq!(remaining_input, expected_remaining_input);
      }

      let failing_test_cases = [
         ("123", Err(Err::Error(Error::new("123", ErrorKind::Tag)))),
         ("{",   Err(Err::Error(Error::new("{",   ErrorKind::Tag)))),
         ("}",   Err(Err::Error(Error::new("}",   ErrorKind::Tag)))),
         ("(",   Err(Err::Error(Error::new("(",   ErrorKind::Tag)))),
         (")",   Err(Err::Error(Error::new(")",   ErrorKind::Tag)))),
         (";",   Err(Err::Error(Error::new(";",   ErrorKind::Tag)))),
      ];

      for (input, expected_error) in failing_test_cases {
         assert_eq!(IdentifierToken::parse(input), expected_error);
      }
   }
}
