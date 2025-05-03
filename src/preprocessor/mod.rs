use std::fs;
use std::io;
use std::process::Command;

pub struct Preprocessor {
   source_filename: String,
   pp_filename: String,
}

impl Drop for Preprocessor {
   fn drop(&mut self) {
      let _ = std::fs::remove_file(&self.pp_filename);
   }
}

impl Preprocessor {
   pub fn new(source_filename_: String) -> Self {
      Self {
         source_filename: source_filename_.clone(),
         pp_filename: source_filename_.replace(".c", "_pp.c")
      }
   }

   pub fn process(&mut self) -> Result<String, io::Error> {
      let _ = Command::new("gcc")
          .arg("-E")
          .arg("-P")
          .arg(&self.source_filename)
          .arg("-o")
          .arg(&self.pp_filename)
          .output();

      fs::read_to_string(&self.pp_filename)
   }
}