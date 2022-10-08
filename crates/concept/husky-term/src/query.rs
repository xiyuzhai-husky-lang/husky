use crate::InternTerm;

#[salsa::query_group(TermQueryStorage)]
pub trait TermQuery: InternTerm {}
