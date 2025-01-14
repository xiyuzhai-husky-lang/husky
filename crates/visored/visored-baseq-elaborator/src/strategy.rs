pub mod obvious;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqStrategy {
    Obvious,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqStrategyCall {
    Obvious,
}

impl VdBsqStrategyCall {
    pub fn wrap<'db, 'sess, R>(self, m: impl ElabM<'db, 'sess, R>) -> impl ElabM<'db, 'sess, R>
    where
        'db: 'sess,
    {
        call::stack::with_call(self, m)
    }
}
