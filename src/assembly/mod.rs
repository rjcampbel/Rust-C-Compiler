pub mod at;

use at::Program;
use crate::tacky::tacky_ast;

pub struct AssemblyGen {
   program: tacky_ast::Program
}

impl AssemblyGen {
   pub fn new(program_: tacky_ast::Program) -> Self {
      Self {
         program: program_
      }
   }

   pub fn parse(&mut self) -> Result<Program, String> {
      Program::parse(&self.program)
   }
}