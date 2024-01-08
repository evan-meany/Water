use crate::parser::NodeProgram;

use std::fs::File;
use std::io::Write;

pub struct Generator {
   program: NodeProgram
}

impl Generator {
   pub fn new(program: NodeProgram) -> Self {
      return Generator{program: program};
   }

   pub fn generate(&self, filename: &str) -> Result<(), String> {
      let mut file = match File::create(filename) {
         Ok(result) => result,
         Err(err) => return Err(String::from("Could not create file: {err}"))
      };

      match file.write_all(b"    mov rax, 60;\n    mov rdi, 0;\n    syscall") {
         Ok(_) => {}
         Err(err) => return Err(String::from("Could not write to file: {err}"))
      }

      match file.flush() {
         Ok(_) => {}
         Err(err) => return Err(String::from("Could not flush file: {err}"))
      }

      return Ok(())
   }
}