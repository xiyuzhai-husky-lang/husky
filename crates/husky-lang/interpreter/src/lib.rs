mod any;
mod builtin;
mod error;
mod opn;
mod utils;

#[cfg(test)]
mod tests;

pub use any::Any;
pub use builtin::*;
pub use error::RuntimeError;

use ast::AstQuery;

pub trait Interpreter: AstQuery {
    fn exec(main_file: file::FileId) -> Result<(), RuntimeError> {
        todo!()
    }
}
