mod context;

pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(jar = TermJar)]
pub struct TermCurry {
    pub curry_variant: TermCurryVariant,
    pub x: Term,
    pub y: Term,
    // ty: Term,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TermCurryVariant {
    Physical {
        physical_curry_kind: TermPhysicalCurryVariant,
        modifier: PhysicalParameterModifier,
    },
    Conceptual,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TermPhysicalCurryVariant {
    Fp,
    Closure {
        opt_ctx: Option<TermCurryContext>,
        lifetimes: Vec<Lifetime>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Lifetime(Term);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PhysicalParameterModifier {
    None,
    Move,
    MoveMut,
}

impl<'a> TermContext<'a> {
    pub(crate) fn curry(
        &self,
        _curry_kind: TermCurryVariant,
        _x: Term,
        _y: Term,
    ) -> TermResult<Term> {
        todo!()
        // TODO: check type
        // Term::new(
        //     self.it_term(
        //         TermCurry {
        //             curry_variant: curry_kind,
        //             x,
        //             y,
        //             // ty: Term::new(self.sort(x.universe().max(y.universe())))?,
        //         }
        //         .into(),
        //     ),
        // )
        // if self.ty_family(x)? == TyFamily::Monadic {
        //     return Err(TermError::MonadIsNotInput);
        // }
        // msg_once!("check compatibility of y");
    }
}

// #[test]
// fn test_curry() {
//     todo!()
//     // let db = TermTestsDb::new();
//     // let menu = db.term_menu();
//     // let ctx = TermContext::new(&db, &menu);
//     // let i32_to_i32 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.i32(), menu.i32())
//     //     .unwrap();
//     // assert_eq!(i32_to_i32.to_string(), "i32 -> i32");
//     // let i64_to_i64 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.i64(), menu.i64())
//     //     .unwrap();
//     // assert_eq!(i64_to_i64.to_string(), "i64 -> i64");
//     // let f32_to_f32 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.f32(), menu.f32())
//     //     .unwrap();
//     // assert_eq!(f32_to_f32.to_string(), "f32 -> f32");
//     // let f64_to_f64 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.f64(), menu.f64())
//     //     .unwrap();
//     // assert_eq!(f64_to_f64.to_string(), "f64 -> f64");
//     // let r32_to_r32 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.r32(), menu.r32())
//     //     .unwrap();
//     // assert_eq!(r32_to_r32.to_string(), "r32 -> r32");
//     // let b64_to_b64 = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.b64(), menu.b64())
//     //     .unwrap();
//     // assert_eq!(b64_to_b64.to_string(), "b64 -> b64");
//     // let bool_to_bool = ctx
//     //     .curry(TermCurryVariant::Conceptual, menu.bool(), menu.bool())
//     //     .unwrap();
//     // assert_eq!(bool_to_bool.to_string(), "bool -> bool");
// }

impl Into<Term> for TermCurry {
    fn into(self) -> Term {
        Term::Curry(self)
    }
}
