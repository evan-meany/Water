use crate::parser::NodeProgram;
use std::fs::File;
use std::io::Write;
use std::process::Command;

static OBJ_FILE: &'static str = "files/test.o";
static EXE_FILE: &'static str = "files/test.exe";
static EXE_COMMAND: &'static str = "./files/test.exe";

pub struct Generator {
    program: NodeProgram,
}

impl Generator {
    pub fn new(program: NodeProgram) -> Self {
        Generator { program }
    }

    pub fn generate(&self, filename: &str) -> Result<(), String> {
        let mut file = File::create(filename)
            .map_err(|err| format!("Could not create file: {}", err))?;

        let asm_code = b"          global    _start

section   .text
_start:   mov       rax, 1                  ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
          mov       rsi, message            ; address of string to output
          mov       rdx, 13                 ; number of bytes
          syscall                           ; invoke operating system to do the write
          mov       rax, 60                 ; system call for exit
          xor       rdi, rdi                ; exit code 0
          syscall                           ; invoke operating system to exit

section   .data
message:  db        \"Hello, World\", 10      ; note the newline at the end";

        file.write_all(asm_code)
            .map_err(|err| format!("Could not write to file: {}", err))?;

        file.flush()
            .map_err(|err| format!("Could not flush file: {}", err))?;

        Ok(())
    }

    pub fn run(&self, output_path: &str) {
        let output = Command::new("nasm")
        .args(["-f", "elf64", output_path, "-o", OBJ_FILE])
        .output()
        .expect("Failed to execute NASM");

        if !output.status.success() {
            eprintln!("NASM Error: {}", String::from_utf8_lossy(&output.stderr));
            return;
        }

        // Linking the object file to create an executable
        let output = Command::new("ld")
            .args(["-o", EXE_FILE, OBJ_FILE])
            .output()
            .expect("Failed to link object file");

        if !output.status.success() {
            eprintln!("Linking Error: {}", String::from_utf8_lossy(&output.stderr));
            return;
        }

        // Executing the compiled executable
        let output = Command::new(EXE_COMMAND)
            .output()
            .expect("Failed to execute the compiled executable");

        if !output.status.success() {
            eprintln!("Execution Error: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("Execution Output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
