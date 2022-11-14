use crate::*;

pub(crate) fn ask_is_input_output_okay<Input, Output>(input: &Input, output: &Output) -> bool
where
    Input: for<'a> Deserialize<'a>
        + Serialize
        + std::fmt::Display
        + std::fmt::Debug
        + PartialEq
        + Eq
        + std::ops::Deref,
    Output:
        for<'a> Deserialize<'a> + Serialize + std::fmt::Display + std::fmt::Debug + PartialEq + Eq,
{
    ask_yes_or_no(format!(
        r#"
{CYAN}[input]{RESET}
{}
{CYAN}[output]{RESET}
{}

is this okay (y/n)? "#,
        textwrap::indent(&input.to_string(), "    ").blue(),
        textwrap::indent(&output.to_string(), "    ").yellow(),
    ))
}

fn ask_yes_or_no(message: String) -> bool {
    loop {
        print!("{message}");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        match &s as &str {
            "y" => break true,
            "n" => break false,
            _ => println!("Invalid answer: {}", s),
        }
    }
}
