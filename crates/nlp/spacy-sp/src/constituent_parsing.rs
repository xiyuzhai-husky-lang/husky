mod subprocess;

// fn main() -> std::io::Result<()> {
//     // Assuming `python_server.py` is in the current directory and Python is `python3`.
//     let mut parser = ConstituentParsingSubprocess::new("python3", "python_server.py")?;

//     // First request
//     let response = parser.parse_text("This is a sentence to parse.").unwrap();
//     println!("Response: {}", response);

//     // Another request
//     let another_response = parser.parse_text("Another sentence here.").unwrap();
//     println!("Another response: {}", another_response);

//     Ok(())
// }
