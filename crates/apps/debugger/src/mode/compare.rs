use std::{
    fmt::Debug,
    fs,
    io::{stdin, stdout, Write},
    path::Path,
};

pub(super) fn compare_saved_data<T>(new_data: &T, saved_data_path: &Path)
where
    T: Debug + Default + PartialEq,
{
    let new_data_text = to_string(new_data);
    let data_on_disk_text: String = if !saved_data_path.exists() {
        "nothing".into()
    } else {
        fs::read_to_string(&saved_data_path).unwrap()
    };
    if data_on_disk_text != new_data_text {
        notify_change(&new_data_text, &data_on_disk_text, &saved_data_path)
    }
}

fn notify_change(new_data_text: &str, old_data_text: &str, save_path: &Path) {
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
    } else {
        panic!("Change in saved data not accepted")
    }
}

fn to_string<T>(data: &T) -> String
where
    T: Debug,
{
    return format!("{:#?}", data);
}
