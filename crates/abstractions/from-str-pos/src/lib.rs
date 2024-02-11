use std::{borrow::Cow, str::FromStr};

pub trait FromStrPos: FromStr {
    fn to_err(e: Self::Err, pos: usize) -> FromStrPosParseError;
}
impl FromStrPos for u8 {
    fn to_err(_: Self::Err, pos: usize) -> FromStrPosParseError {
        FromStrPosParseError::BadInteger(pos)
    }
}
impl FromStrPos for u32 {
    fn to_err(_: Self::Err, pos: usize) -> FromStrPosParseError {
        FromStrPosParseError::BadInteger(pos)
    }
}
impl FromStrPos for usize {
    fn to_err(_: Self::Err, pos: usize) -> FromStrPosParseError {
        FromStrPosParseError::BadInteger(pos)
    }
}

#[derive(Debug)]
pub enum FromStrPosParseError {
    UnexpectedElement {
        pos: usize,
        expected: &'static str,
        found: Option<Cow<'static, str>>,
    },
    BadInteger(usize),
}
