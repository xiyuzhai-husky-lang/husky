use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct ConstituentParser {
    child: Child,
    child_stdin: ChildStdin,
    child_stdout: BufReader<ChildStdout>,
}

impl ConstituentParser {
    /// Spawn the Python subprocess and return an NLPParser instance.
    pub fn new(python_path: &str, server_script: &str) -> std::io::Result<Self> {
        let mut child = Command::new(python_path)
            .arg(server_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit()) // For debugging
            .spawn()?;

        let child_stdin = child.stdin.take().expect("Failed to open child stdin");
        let child_stdout = child.stdout.take().expect("Failed to open child stdout");

        Ok(ConstituentParser {
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

impl Drop for ConstituentParser {
    /// Ensure that when NLPParser is dropped, we terminate the subprocess.
    fn drop(&mut self) {
        // Attempt to terminate the child process gracefully.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn main() -> std::io::Result<()> {
    // Assuming `python_server.py` is in the current directory and Python is `python3`.
    let mut parser = ConstituentParser::new("python3", "python_server.py")?;

    // First request
    let response = parser.parse_text("This is a sentence to parse.").unwrap();
    println!("Response: {}", response);

    // Another request
    let another_response = parser.parse_text("Another sentence here.").unwrap();
    println!("Another response: {}", another_response);

    Ok(())
}
