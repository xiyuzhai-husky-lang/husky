use serde::Serialize;
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
};

pub(super) fn notify_change<T>(new: T, old: T, save_path: &Path)
where
    T: std::fmt::Debug + Serialize,
{
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
    print!("old = \n  {:?}\n", &old);
    print!("new = \n  {:?}\n", &new);
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
        fs::write(save_path, serde_json::to_string(&new).unwrap()).expect("Error writing");
    } else {
        panic!("Change in saved data not accepted")
    }
}

pub(super) fn notify_deserialize_error<T>(
    new: T,
    old_text: &str,
    e: &serde_json::error::Error,
    save_path: &Path,
) where
    T: std::fmt::Debug + Serialize,
{
    // notify the difference between the old and the new
    // ask whether to update the old
    println!(
        "{}Unable to deserialize saved data{} for file {}{:?}{},\n{}error{}:\n  {:?}",
        print_utils::RED,
        print_utils::RESET,
        print_utils::GREEN,
        save_path.as_os_str(),
        print_utils::RESET,
        print_utils::RED,
        print_utils::RESET,
        e
    );
    print!(
        "{}old text{} = \n  {:?}\n",
        print_utils::CYAN,
        print_utils::RESET,
        &old_text
    );
    print!(
        "{}new{} = \n  {:?}\n",
        print_utils::CYAN,
        print_utils::RESET,
        &new
    );
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
        fs::write(save_path, serde_json::to_string(&new).unwrap()).expect("Error writing");
    } else {
        panic!("Change in saved data not accepted")
    }
}
