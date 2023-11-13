use thiserror::Error;

#[derive(Debug, Error)]
#[error("insert entry repeat error {old}")]
pub struct InsertEntryRepeatError<Entry> {
    pub old: usize,
    pub new: Entry,
}

#[derive(Debug, Error)]
#[error("from vec entry repeat error {i} {j}")]
pub struct FromVecEntryRepeatError {
    pub i: usize,
    pub j: usize,
}
