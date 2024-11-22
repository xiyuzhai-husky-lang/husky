pub mod error;
pub mod expr;
pub mod file;
pub mod helpers;
mod parser;
pub mod row;
mod token;

use self::{error::*, expr::*, file::*, parser::*, row::*};
#[cfg(test)]
use expect_test::*;
use ordered_float::OrderedFloat;

pub fn parse_lp_csv_file(s: &str) -> Result<LpCsvFile, LpCsvError> {
    LpCsvParser::new(s).parse_file()
}
