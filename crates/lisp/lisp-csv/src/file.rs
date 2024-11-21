use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpCsvFile {
    Rows(Vec<LpCsvRow>),
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn parse_file(mut self) -> LpCsvResult<LpCsvFile> {
        let mut rows = Vec::new();
        while let Some(row) = self.parse_row()? {
            rows.push(row);
        }
        Ok(LpCsvFile::Rows(rows))
    }
}
