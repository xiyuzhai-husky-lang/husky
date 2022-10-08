use std::ops::Deref;

use crate::*;

/// representing term `x -> y`
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurry {
    kind: TermCurryKind,
    x: Ty,
    y: Ty,
    ty: Ty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TermCurryKind {
    Physics {
        physical_curry_kind: TermPhysicalCurryKind,
        modifier: PhysicalParameterModifier,
    },
    Concept,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TermPhysicalCurryKind {
    Fp,
    PartialFp,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PhysicalParameterModifier {
    None,
    Move,
    MoveMut,
}

impl TermCurry {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn new(ctx: &TermContext, kind: TermCurryKind, x: Ty, y: Ty) -> TermResult<TermPtr> {
        if ctx.ty_family(x)? == TyFamily::Monad {
            return Err(TermError::MonadIsNotInput);
        }
        Ok(ctx.it_term(
            TermCurry {
                kind,
                x,
                y,
                ty: Ty::new(ctx.it_term(TermUniverse::ty_universe(
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
