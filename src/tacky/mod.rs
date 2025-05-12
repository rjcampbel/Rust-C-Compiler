pub mod tacky_ast;

use crate::parser::ast;

struct Tacky {
   ast: ast::Program
}

impl Tacky {
   pub fn new(ast_: ast::Program) -> Self {
      Tacky {
         ast: ast_
      }
   }

   pub fn generate() -> Result<tacky_ast::Program, String> {
      Err(String::from("not yet implemented"))
   }
}