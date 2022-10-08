use crate::TermQuery;

pub struct TermContext<'a> {
    db: &'a dyn TermQuery,
}
