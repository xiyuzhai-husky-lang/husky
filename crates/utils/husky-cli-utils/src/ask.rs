use colored::Colorize;
use husky_screen_utils::light_up_screen;
use std::io::Write;

pub fn ask_user_for_permission(question: impl std::fmt::Display) -> bool {
    light_up_screen();
    loop {
        print!("{}? (y/n)", question.to_string().blue());
        let _ = std::io::stdout().flush();
        let mut s = String::new();
        std::io::stdin()
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
