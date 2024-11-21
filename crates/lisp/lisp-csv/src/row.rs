use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvRow {
    Value(LpCsvExpr),
    SeparatedValues(Vec<LpCsvExpr>),
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_row(&mut self) -> LpCsvResult<Option<LpCsvRow>> {
        todo!()
    }
}
