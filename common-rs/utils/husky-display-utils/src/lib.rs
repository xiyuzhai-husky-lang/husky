use text_diff::print_diff;

use husky_test_utils::TestResult;
use std::fmt::Write;
use std::io::Write as _;
use std::{
    fmt::Debug,
    fs,
    io::{stdin, stdout},
    path::Path,
    sync::Arc,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HuskyDisplayConfig {
    pub colored: bool,
    pub indent: u8,
}

impl HuskyDisplayConfig {
    pub fn indented(&self) -> Self {
        Self {
            colored: self.colored,
            indent: self.indent + 4,
        }
    }
}

pub trait HuskyDisplay: std::fmt::Debug + PartialEq {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String);
    fn print_inherent(&self, config: HuskyDisplayConfig) -> String {
        let mut result = String::new();
        self.write_inherent(config, &mut result);
        result
    }
}

impl<T: HuskyDisplay> HuskyDisplay for Option<T> {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        match self {
            Some(value) => value.write_inherent(config, result),
            None => (),
        }
    }
}

impl<T: HuskyDisplay> HuskyDisplay for Arc<T> {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        (**self).write_inherent(config, result)
    }
}

impl<T: HuskyDisplay> HuskyDisplay for Vec<T> {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        for t in self.iter() {
            t.write_inherent(config, result);
            result.push_str("\n");
        }
    }
}

impl<T: HuskyDisplay, S: HuskyDisplay> HuskyDisplay for (T, S) {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        self.0.write_inherent(config, result);
        result.push_str("  ");
        self.1.write_inherent(config, result);
    }
}

impl<T, E> HuskyDisplay for Result<T, E>
where
    T: HuskyDisplay,
    E: HuskyDisplay,
{
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        match self {
            Ok(v) => v.write_inherent(config, result),
            Err(e) => e.write_inherent(config, result),
        }
    }
}

// impl HuskyDisplay for lsp_types::FoldingRange {
//     fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
//         write!(result, "{:?}", self).unwrap();
//     }
// }

pub fn compare_saved_data<T>(new_data: &T, saved_data_path: &Path) -> TestResult
where
    T: HuskyDisplay,
{
    let new_data_text = new_data.print_inherent(HuskyDisplayConfig {
        colored: false,
        indent: 0,
    });
    let data_on_disk_text: String = if !saved_data_path.exists() {
        "".into()
    } else {
        fs::read_to_string(&saved_data_path).unwrap()
    };
    if data_on_disk_text != new_data_text {
        notify_change(&new_data_text, &data_on_disk_text, &saved_data_path)
    } else {
        TestResult::Success
    }
}

fn notify_change(new_data_text: &str, old_data_text: &str, save_path: &Path) -> TestResult {
    // notify the difference between the old and the new
    // ask whether to update the old
    println!(
        "{}Change in saved data{} for file {}{}{},",
        husky_print_utils::MAGENTA,
        husky_print_utils::RESET,
        husky_print_utils::GREEN,
        save_path.as_os_str().to_str().unwrap(),
        husky_print_utils::RESET,
    );
    print_diff(old_data_text, new_data_text, "\n");
    let accept: bool = loop {
        print!("Do you want to accept change in saved data (y/n)? ");
        let mut s = String::new();
        let _ = stdout().flush();
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
    };
    if accept {
        fs::write(save_path, format!("{}", new_data_text)).expect("Error writing");
        TestResult::Success
    } else {
        TestResult::Failure
    }
}
