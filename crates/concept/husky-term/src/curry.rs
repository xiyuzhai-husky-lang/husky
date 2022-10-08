use std::ops::Deref;

use crate::*;

/// representing term `x -> y`
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurry {
    curry_kind: TermCurryKind,
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
}

impl<'a> TermContext<'a> {
    pub fn curry(&self, curry_kind: TermCurryKind, x: Ty, y: Ty) -> TermResult<Ty> {
        if self.ty_family(x)? == TyFamily::Monad {
            return Err(TermError::MonadIsNotInput);
        }
        Ty::new(
            self.it_term(
                TermCurry {
                    curry_kind,
                    x,
                    y,
                    ty: Ty::new(self.it_term(TermUniverse::ty_universe(
                        x.universe_level().max(y.universe_level()),
                    )))?,
                }
                .into(),
            ),
        )
    }
}

#[test]
fn test_curry() {
    let db = TermTestsDb::new();
    let ctx = TermContext::new(&db);
    let menu = db.term_menu();
    assert_eq!(
        ctx.curry(TermCurryKind::Concept, menu.i32(), menu.i32())
            .unwrap()
            .to_string(),
        "i32 -> i32"
    );
}

impl Into<Term> for TermCurry {
    fn into(self) -> Term {
        Term::Curry(self)
    }
}
