use crate::TestResult;
use std::fmt::Write;
use std::io::Write as _;
use std::{
    fmt::Debug,
    fs,
    io::{stdin, stdout},
    path::Path,
    sync::Arc,
};

pub trait TestComparable: std::fmt::Debug + PartialEq {
    fn write_inherent(&self, result: &mut String);
    fn print_inherent(&self) -> String {
        let mut result = String::new();
        self.write_inherent(&mut result);
        result
    }
}

impl<T: TestComparable> TestComparable for Arc<T> {
    fn write_inherent(&self, result: &mut String) {
        (**self).write_inherent(result)
    }
}

impl<T: TestComparable> TestComparable for Vec<T> {
    fn write_inherent(&self, result: &mut String) {
        for t in self.iter() {
            t.write_inherent(result);
            result.push_str("\n");
        }
    }
}

impl<T, E> TestComparable for Result<T, E>
where
    T: TestComparable,
    E: TestComparable,
{
    fn write_inherent(&self, result: &mut String) {
        match self {
            Ok(v) => v.write_inherent(result),
            Err(e) => e.write_inherent(result),
        }
    }
}

impl TestComparable for lsp_types::SemanticToken {
    fn write_inherent(&self, result: &mut String) {
        write!(result, "{:?}", self).unwrap();
    }
}

impl TestComparable for lsp_types::FoldingRange {
    fn write_inherent(&self, result: &mut String) {
        write!(result, "{:?}", self).unwrap();
    }
}

pub fn compare_saved_data<T>(new_data: &T, saved_data_path: &Path) -> TestResult
where
    T: TestComparable,
{
    let new_data_text = new_data.print_inherent();
    let data_on_disk_text: String = if !saved_data_path.exists() {
        "nothing".into()
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
        "{}Change in saved data{} for file {}{:?}{},",
        print_utils::MAGENTA,
        print_utils::RESET,
        print_utils::GREEN,
        save_path.as_os_str(),
        print_utils::RESET,
    );
    print!("old = \n  {}\n", &old_data_text);
    print!("new = \n  {}\n", &new_data_text);
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
        TestResult::Failed
    }
}
