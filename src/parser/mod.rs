mod ast;

use crate::lexer::token::Token;
use ast::Program;

pub struct Parser {
   pp_tokens: Vec<Token>
}

impl Parser {
   pub fn new(pp_tokens_: Vec<Token>) -> Self {
      Parser {
         pp_tokens: pp_tokens_
      }
   }

   pub fn parse(&mut self) -> Result<ast::Program, String> {
      Program::parse(&self.pp_tokens)
   }
}