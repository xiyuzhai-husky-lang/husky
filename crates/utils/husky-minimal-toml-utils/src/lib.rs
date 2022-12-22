use husky_print_utils::p;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum MinimalTomlError {
    #[error("expect word `name`")]
    ExpectWordName,
    #[error("expect operator `=`")]
    ExpectAssign,
    #[error("expect identifier `=`")]
    ExpectIdentifier,
}

pub type MinimalTomlResult<T> = Result<T, MinimalTomlError>;

pub fn find_package_name_in_toml(input: &str) -> MinimalTomlResult<&str> {
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line == "[package]" {
            break;
        }
    }
    while let Some(line) = lines.next() {
        let line = line.trim();
        if let Some(c) = line.chars().next() {
            if !c.is_alphabetic() {
                todo!()
            }
        } else {
            todo!()
        }
        let mut splits = line.split(' ');
        if splits.next() == Some("name") {
            if splits.next() != Some("=") {
                return Err(MinimalTomlError::ExpectAssign);
            }
            let split = splits.next().ok_or(MinimalTomlError::ExpectIdentifier)?;
            if !split.starts_with('"') {
                todo!()
            }
            if !split.ends_with('"') {
                todo!()
            }
            let split = &split[1..(split.len() - 1)];
            return Ok(split);
        }
    }
    Err(MinimalTomlError::ExpectWordName)
}

#[test]
fn find_package_name_in_toml_works() {
    assert_eq!(
        find_package_name_in_toml(
            r#"

[package]
name = "haha"

"#,
        ),
        Ok("haha")
    )
}
