use husky_entity_route::{CanonicalTy, EntityRoutePtr, RangedEntityRoute};
use husky_pattern_semantics::PurePattern;
use husky_print_utils::p;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplicitConversion {
    None,
    WrapInSome { number_of_somes: u8 },
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
            // todo: improve this
            let expectation = expectation.canonicalize();
            if expectation.intrinsic_ty() != ty.intrinsic_ty() {
                p!(expectation.intrinsic_ty(), ty.intrinsic_ty());
                todo!()
            }
            if expectation.qual() != ty.qual() {
                todo!()
            }
            if expectation.option_level() != ty.option_level() {
                match expectation.option_level() > ty.option_level() {
                    true => {
                        return ImplicitConversion::WrapInSome {
                            number_of_somes: expectation.option_level() - ty.option_level(),
                        };
                    }
                    false => todo!(),
                }
            }
            Self::None
        } else {
            Default::default()
        }
    }
}
