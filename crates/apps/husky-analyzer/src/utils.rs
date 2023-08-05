use crate::*;
use serde::de::DeserializeOwned;
use std::{fs::OpenOptions, io::Write};

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = serde_json::from_value(json.clone())
        .map_err(|e| format!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}

const LOG_FILE_PATH: &str = "/home/xiyuzhai/temp/husky-analyzer-server.log";

pub(crate) fn init_log_file() {
    let mut file = std::fs::File::create("/home/xiyuzhai/temp/husky-analyzer-server.log").unwrap();
    file.write_all(b"Hello, world!\n\n").unwrap();
}

pub(crate) fn log_aux(msg: impl AsRef<str>) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(LOG_FILE_PATH)
        .expect("ok");
    file.write_all(b"\n").expect("ok");
    file.write_all(msg.as_ref().as_bytes()).expect("ok");
    file.write_all(b"\n").expect("ok")
}

macro_rules! log {
    ($($arg: expr),*) => {
        crate::utils::log_aux(&format!($($arg),*))
    };
}
pub(crate) use log;

#[test]
fn t() {
    assert_eq!(b"\n".len(), 1)
}
