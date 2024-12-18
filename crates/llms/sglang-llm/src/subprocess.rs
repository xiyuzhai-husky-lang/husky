use lazy_static::lazy_static;
use sglang_llm_prelude::greeting::IPC_GREETING;
use std::{
    io::{self, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::Mutex,
};

pub struct Subprocess {
    child: Child,
    stdin: ChildStdin,
    stdout_lines: Lines<BufReader<ChildStdout>>,
}

impl Subprocess {
    pub fn new() -> Self {
        use std::io::BufRead;

        let mut child = Command::new("docker")
            .args(["run", "--gpus", "all", "-i", "rust-calculator"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Docker container");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let mut stdout_lines = io::BufReader::new(stdout).lines();
        for line in &mut stdout_lines {
            let line = line.unwrap();
            println!("SGLANG-LLM-SUBPROCESS: {}", line);
            if line.contains(IPC_GREETING) {
                break;
            }
        }
        Self {
            child,
            stdin,
            stdout_lines,
        }
    }
}

impl Drop for Subprocess {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill() {
            eprintln!("Warning: Failed to kill Docker container: {}", e);
        }
        if let Err(e) = self.child.wait() {
            eprintln!(
                "Warning: Error waiting for Docker container to terminate: {}",
                e
            );
        }
    }
}

lazy_static! {
    pub(crate) static ref SUBPROCESS: Mutex<Subprocess> = Mutex::new(Subprocess::new());
}

impl Subprocess {
    pub fn write_line(&mut self, line: impl AsRef<str>) {
        use std::io::Write;

        writeln!(self.stdin, "{}", line.as_ref()).expect("Failed to write to stdin");
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        use std::io::BufRead;

        self.stdout_lines.next().unwrap()
    }
}
