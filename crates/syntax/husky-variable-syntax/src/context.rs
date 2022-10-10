use crate::VariableQuery;

pub struct VariableContext<'a> {
    db: &'a dyn VariableQuery,
}

impl<'a> VariableContext<'a> {
    pub fn new(db: &'a dyn VariableQuery) -> Self {
        Self { db }
    }
}
