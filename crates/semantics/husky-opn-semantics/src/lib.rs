use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_pattern_semantics::PurePattern;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerSuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
    BePattern(PurePattern),
}

impl EagerSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            EagerSuffixOpr::Incr => "++".into(),
            EagerSuffixOpr::Decr => "--".into(),
            EagerSuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
            EagerSuffixOpr::BePattern(_) => todo!(),
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
    pub fn from_opt_expectation(
        opt_expectation: Option<EntityRoutePtr>,
        ty: EntityRoutePtr,
    ) -> Self {
        if let Some(expectation) = opt_expectation {
            if expectation == ty {
                return Default::default();
            }

            if expectation.is_option() && expectation.spatial_arguments[0].take_entity_route() == ty
            {
                return ImplicitConversion::WrapInSome;
            }

            todo!()
        } else {
            Default::default()
        }
    }
}
