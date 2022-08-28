use husky_entity_route::{CanonicalTy, EntityRoutePtr, RangedEntityRoute};
use husky_pattern_semantics::PurePattern;
use husky_print_utils::p;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerSuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
    BePattern(PurePattern),  // be <patt>
    Unveil,                  // ?
}

impl EagerSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            EagerSuffixOpr::Incr => "++".into(),
            EagerSuffixOpr::Decr => "--".into(),
            EagerSuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
            EagerSuffixOpr::BePattern(_) => todo!(),
            EagerSuffixOpr::Unveil => "?".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplicitConversion {
    None,
    WrapInSome,
    ConvertToBool,
}

impl Default for ImplicitConversion {
    fn default() -> Self {
        Self::None
    }
}

impl ImplicitConversion {
    pub fn from_opt_expectation(opt_expectation: Option<EntityRoutePtr>, ty: &CanonicalTy) -> Self {
        if let Some(expectation) = opt_expectation {
            let expectation = expectation.canonicalize();
            if expectation.intrinsic_ty() != ty.intrinsic_ty() {
                p!(expectation.intrinsic_ty(), ty.intrinsic_ty());
                todo!()
            }
            if expectation.qual() != ty.qual() {
                todo!()
            }
            if expectation.is_option() != ty.is_option() {
                todo!()
            }
            Self::None
            // match canonical_expectation.kind() {
            //     CanonicalEntityRouteKind::Intrinsic => {
            //         if expectation == ty {
            //             return Default::default();
            //         }
            //         todo!()
            //     }
            //     CanonicalEntityRouteKind::Optional => todo!(),
            //     CanonicalEntityRouteKind::EvalRef => todo!(),
            //     CanonicalEntityRouteKind::OptionalEvalRef => match canonical_ty.kind() {
            //         CanonicalEntityRouteKind::Intrinsic => {
            //             todo!()
            //         }
            //         CanonicalEntityRouteKind::Optional => todo!(),
            //         CanonicalEntityRouteKind::EvalRef => todo!(),
            //         CanonicalEntityRouteKind::OptionalEvalRef => todo!(),
            //         CanonicalEntityRouteKind::TempRefMut => todo!(),
            //     },
            //     CanonicalEntityRouteKind::TempRefMut => todo!(),
            // }
            // if expectation == ty {
            //     return Default::default();
            // }

            // if canonical_expectation.is_option() && expectation.entity_route_argument(0) == ty {
            //     return ImplicitConversion::WrapInSome;
            // }

            // p!(expectation, ty);
        } else {
            Default::default()
        }
    }
}
