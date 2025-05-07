pub mod at;

use at::Program;
use crate::parser::ast;

pub struct AssemblyGen {
   program: ast::Program
}

impl AssemblyGen {
   pub fn new(program_: ast::Program) -> Self {
      Self {
         program: program_
      }
   }

   pub fn parse(&mut self) -> Result<Program, String> {
      Program::parse(&self.program)
   }
}