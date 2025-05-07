use std::fs;
use std::io;
use std::process::Command;

pub struct Assembler {
   s_filename: String,
   output_filename: String
}

impl Drop for Assembler {
   fn drop(&mut self) {
      _ = std::fs::remove_file(&self.s_filename);
   }
}

impl Assembler {
   pub fn new(s_filename_: &String) -> Self {
      Self {
         s_filename: s_filename_.clone(),
         output_filename: s_filename_.replace(".s", "")
      }
   }

   pub fn process(&mut self) -> Result<(), io::Error> {
      let _ = Command::new("gcc")
          .arg(&self.s_filename)
          .arg("-o")
          .arg(&self.output_filename)
          .output();

      fs::read_to_string(&self.s_filename)?;
      Ok(())
   }
}