use std::ops::Deref;

use crate::*;

/// representing term `x -> y`
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurry {
    x: Ty,
    y: Ty,
    ty: Ty,
}

impl TermCurry {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn new(ctx: &TermContext, x: Ty, y: Ty) -> TermResult<TermPtr> {
        let ty = todo!();
        Ok(ctx.db.it_term(
            TermCurry {
                x: todo!(),
                y: todo!(),
                ty: Ty::new(ctx.db.it_term(TermUniverse::ty_universe(
                    x.universe_level().max(y.universe_level()),
                )))?,
            }
            .into(),
        ))
    }
}

impl Into<Term> for TermCurry {
    fn into(self) -> Term {
        Term::Curry(self)
    }
}
