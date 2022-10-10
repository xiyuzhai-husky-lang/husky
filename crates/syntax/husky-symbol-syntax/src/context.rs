use crate::VariableQuery;

pub struct SymbolContext<'a> {
    db: &'a dyn VariableQuery,
}

impl<'a> SymbolContext<'a> {
    pub fn new(db: &'a dyn VariableQuery) -> Self {
        Self { db }
    }
}
