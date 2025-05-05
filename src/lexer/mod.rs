pub mod token;

use token::Token;

pub struct Lexer {
   pp_source: String
}

impl Lexer {
   pub fn new(pp_source_: String) -> Self {
      Self {
         pp_source: pp_source_
      }
   }

   pub fn lex(&mut self) -> Result<Vec<Token>, String> {
      let mut tokens: Vec<Token> = Vec::new();
      let mut chars = self.pp_source.chars().peekable();
      while let Some(c) = chars.next() {
         match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ';' => tokens.push(Token::Semicolon),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            _ if c.is_alphabetic() || c == '_' => {
               let mut token_value: String = String::from(c);
               while let Some(t) = chars.peek() {
                  if t.is_alphanumeric() || *t == '_' {
                     token_value.push(chars.next().unwrap());
                  } else {
                     match token_value.as_str() {
                        "int" => tokens.push(Token::Int),
                        "void" => tokens.push(Token::Void),
                        "return" => tokens.push(Token::Return),
                        _ => tokens.push(Token::Identifier(token_value))
                     }
                     break;
                  }
               }
            },
            _ if c.is_ascii_digit() => {
               let mut token_value: String = String::from(c);
               while let Some(t) = chars.peek() {
                  if t.is_ascii_digit() {
                     token_value.push(chars.next().unwrap());
                  } else {
                     break;
                  }
               }
               if let Some(t) = chars.peek() {
                  if t.is_alphabetic() {
                     return Err(String::from("Invalid identifier"));
                  }
               }
               tokens.push(Token::Integer(token_value.parse::<u64>().unwrap()));
            },
            _ if c.is_whitespace() => (),
            _ => {
               return Err(String::from("Invalid Token"));
            }
         }
      }
      Ok(tokens)
   }
}