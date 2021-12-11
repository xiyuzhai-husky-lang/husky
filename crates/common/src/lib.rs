pub mod _session;
pub mod error;
pub mod print;
pub mod show;
pub mod todo;

pub use error::*;
pub use print::*;
pub use show::*;
pub use std::cell::Cell;
pub use std::collections::HashMap;
pub use std::fmt;
pub use std::fmt::Debug;
pub use std::fmt::Formatter;
pub use std::io;
pub use std::path::{Path, PathBuf};
pub use todo::*;

pub use text_size::{TextRange, TextSize};

#[derive(Debug)]
pub enum ParserErrorVariant {
    SyntaxError(SyntaxError),
    SemanticError(SemanticError),
    IOError(io::Error),
}
#[derive(Debug)]
pub enum SyntaxError {
    IndentError,
    KeywordMisplacement,
    IncompatibleConvexity,
    WrongKeyword,
}

#[derive(Debug)]
pub enum SemanticError {
    TypeResolveFailure,
    EntityResolveFailure,
}

#[derive(Debug)]
pub struct ParserCall {
    pub file: &'static str,
    pub line: u32,
    pub message: String,
}

// #[derive(Debug)]
// pub struct ParserError {
//     pub call_stack: Vec<ParserCall>,
//     pub range: file::Range,
//     pub variant: ParserErrorVariant,
// }
// impl ParserError {
//     // fn new(variant: ParserErrorVariant) -> ParserError {
//     //     td!()
//     // }
//     fn push_call(mut self, file: &'static str, line: u32, message: String) -> ParserError {
//         self.call_stack.push(ParserCall {
//             file,
//             line,
//             message,
//         });
//         self
//     }
// }

#[macro_export]
macro_rules! syntax_error {
    ($range:expr, $message:expr, $variant:ident) => {{
        Err(ParserError {
            call_stack: vec![ParserCall {
                file: file!(),
                line: line!(),
                message: $message,
            }],
            range: $range,
            variant: ParserErrorVariant::SyntaxError(SyntaxError::$variant),
        })
    }};
}

#[macro_export]
macro_rules! semantic_error {
    ($range:expr, $message:expr, $variant:ident) => {{
        Err(ParserError {
            call_stack: vec![ParserCall {
                file: file!(),
                line: line!(),
                message: $message,
            }],
            range: $range,
            variant: ParserErrorVariant::SemanticError(SemanticError::$variant),
        })
    }};
}

#[macro_export]
macro_rules! io_error {
    ($e:expr, $message:expr ) => {{
        Err(ParserError {
            call_stack: vec![ParserCall {
                file: file!(),
                line: line!(),
                message: $message,
            }],
            range: Range::none(),
            variant: ParserErrorVariant::IOError($e),
        })
    }};
}

// pub fn push_call<T>(
//     result: Result<T, ParserError>,
//     file: &'static str,
//     line: u32,
//     message: String,
// ) -> Result<T, ParserError> {
//     match result {
//         Ok(stuff) => Ok(stuff),
//         Err(e) => Err(e.push_call(file, line, message)),
//     }
// }
