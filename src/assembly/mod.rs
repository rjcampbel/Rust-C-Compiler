pub mod at;
pub mod stack_allocator;

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
      let mut program = Program::parse(&self.program)?;
      program.replace_pseudoregs();
      program.register_fixup();
      Ok(program)
   }
}