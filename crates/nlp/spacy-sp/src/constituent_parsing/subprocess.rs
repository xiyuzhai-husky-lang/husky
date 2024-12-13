use serde_json::Value;

use std::io::{BufRead, BufReader, Write};

use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct ConstituentParsingSubprocess {
    child: Child,
    child_stdin: ChildStdin,
    child_stdout: BufReader<ChildStdout>,
}

impl ConstituentParsingSubprocess {
    /// Spawn the Python subprocess and return an NLPParser instance.
    pub fn new(python_path: &str, server_script_path: &str) -> std::io::Result<Self> {
        let mut child = Command::new(python_path)
            .arg(server_script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // For debugging - connects Python process stderr to parent Rust process stderr
            // This means Python errors and debug output will be visible in terminal
            // Useful during development for:
            // 1. Seeing Python errors and print statements directly
            // 2. Not losing subprocess error messages
            // 3. Getting real-time output as Python runs
            // Consider alternatives for production:
            // - Stdio::piped() to capture errors
            // - Redirect to log file
            // - Stdio::null() to suppress
            .spawn()?;

        let child_stdin = child.stdin.take().expect("Failed to open child stdin");
        let child_stdout = child.stdout.take().expect("Failed to open child stdout");

        Ok(ConstituentParsingSubprocess {
            child,
            child_stdin,
            child_stdout: BufReader::new(child_stdout),
        })
    }

    /// Send a request to the Python process and read the response.
    ///
    /// The request should be a JSON object, for example:
    /// {
    ///     "text": "This is a sentence to parse."
    /// }
    ///
    /// The response is expected to be a JSON object as well.
    pub fn parse_text(&mut self, text: &str) -> serde_json::Result<Value> {
        // Create the request JSON
        let request = serde_json::json!({ "text": text });

        // Write the request followed by a newline
        writeln!(self.child_stdin, "{}", request.to_string())
            .expect("Failed to write to child stdin");
        self.child_stdin
            .flush()
            .expect("Failed to flush child stdin");

        // Read one line of response
        let mut response_line = String::new();
        self.child_stdout
            .read_line(&mut response_line)
            .expect("Failed to read line from child stdout");

        // Parse the JSON response
        serde_json::from_str(&response_line)
    }
}

impl Drop for ConstituentParsingSubprocess {
    /// Ensure that when ConstituentParsingSubprocess is dropped, we terminate the subprocess.
    fn drop(&mut self) {
        // Attempt to terminate the child process gracefully.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
