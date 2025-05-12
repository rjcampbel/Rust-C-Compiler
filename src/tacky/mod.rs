pub mod tacky_ast;

use crate::parser::ast;

pub struct Tacky {
   ast: ast::Program
}

impl Tacky {
   pub fn new(ast_: ast::Program) -> Self {
      Tacky {
         ast: ast_
      }
   }

   pub fn generate(&mut self) -> Result<tacky_ast::Program, String> {
      let tacky_ast = tacky_ast::Program::parse(&self.ast)?;
      Ok(tacky_ast)
   }
}