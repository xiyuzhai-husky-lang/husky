use crate::TermQuery;

pub struct TermContext<'a> {
    pub(crate) db: &'a dyn TermQuery,
}
