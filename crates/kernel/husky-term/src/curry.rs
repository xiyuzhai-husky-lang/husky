use std::ops::Deref;

use husky_print_utils::msg_once;

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
    pub fn curry_kind(&self) -> TermCurryKind {
        self.curry_kind
    }

    pub fn x(&self) -> Ty {
        self.x
    }

    pub fn y(&self) -> Ty {
        self.y
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }
}

impl<'a> TermContext<'a> {
    pub fn curry(&self, curry_kind: TermCurryKind, x: Ty, y: Ty) -> TermResult<Ty> {
        if self.ty_family(x)? == TyFamily::Monadic {
            return Err(TermError::MonadIsNotInput);
        }
        msg_once!("check compatibility of y");
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
    let i32_to_i32 = ctx
        .curry(TermCurryKind::Concept, menu.i32(), menu.i32())
        .unwrap();
    assert_eq!(i32_to_i32.to_string(), "i32 -> i32");
    let i64_to_i64 = ctx
        .curry(TermCurryKind::Concept, menu.i64(), menu.i64())
        .unwrap();
    assert_eq!(i64_to_i64.to_string(), "i64 -> i64");
    let f32_to_f32 = ctx
        .curry(TermCurryKind::Concept, menu.f32(), menu.f32())
        .unwrap();
    assert_eq!(f32_to_f32.to_string(), "f32 -> f32");
    let f64_to_f64 = ctx
        .curry(TermCurryKind::Concept, menu.f64(), menu.f64())
        .unwrap();
    assert_eq!(f64_to_f64.to_string(), "f64 -> f64");
    let b32_to_b32 = ctx
        .curry(TermCurryKind::Concept, menu.b32(), menu.b32())
        .unwrap();
    assert_eq!(b32_to_b32.to_string(), "b32 -> b32");
    let b64_to_b64 = ctx
        .curry(TermCurryKind::Concept, menu.b64(), menu.b64())
        .unwrap();
    assert_eq!(b64_to_b64.to_string(), "b64 -> b64");
    let bool_to_bool = ctx
        .curry(TermCurryKind::Concept, menu.bool(), menu.bool())
        .unwrap();
    assert_eq!(bool_to_bool.to_string(), "bool -> bool");
}

impl Into<Term> for TermCurry {
    fn into(self) -> Term {
        Term::Curry(self)
    }
}
