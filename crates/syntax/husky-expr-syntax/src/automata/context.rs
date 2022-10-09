use crate::*;

pub(crate) struct AutomataContext<'a> {
    db: &'a dyn ExprSyntaxQuery,
}

impl<'a> AutomataContext<'a> {
    pub(crate) fn new(db: &'a dyn ExprSyntaxQuery) -> Self {
        Self { db }
    }
}
