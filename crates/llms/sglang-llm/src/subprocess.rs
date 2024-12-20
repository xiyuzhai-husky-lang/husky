use sglang_llm_prelude::greeting::IPC_GREETING;
use std::{
    io::{self, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::Mutex,
};

pub struct SglangLlmSubprocess {
    child: Child,
    stdin: ChildStdin,
    stdout_lines: Lines<BufReader<ChildStdout>>,
}

impl SglangLlmSubprocess {
    pub fn new() -> Self {
        use std::io::BufRead;

        const DOCKER_IMAGE: &str = "sglang-llm-server";

        // First, build the Docker image
        let build_status = Command::new("docker")
            .args(["build", "-t", DOCKER_IMAGE, "."])
            .status()
            .expect("Failed to build Docker image");

        if !build_status.success() {
            panic!("Failed to build Docker image");
        }

        // Then run the container as before
        let mut child = Command::new("docker")
            .args(["run", "--gpus", "all", "-i", DOCKER_IMAGE])
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

impl Drop for SglangLlmSubprocess {
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

impl SglangLlmSubprocess {
    pub fn write_line(&mut self, line: impl AsRef<str>) {
        use std::io::Write;

        if let Err(e) = writeln!(self.stdin, "{}", line.as_ref()) {
            // Check if the process is still running
            match self.child.try_wait() {
                Ok(Some(status)) => panic!("Container exited with status: {}", status),
                Ok(None) => panic!("Failed to write to stdin (container still running): {}", e),
                Err(e) => panic!("Failed to check container status: {}", e),
            }
        }
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        use std::io::BufRead;

        self.stdout_lines.next().unwrap()
    }
}
