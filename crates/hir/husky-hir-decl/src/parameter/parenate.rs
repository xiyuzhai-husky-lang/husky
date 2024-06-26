pub mod eager;
pub mod lazy;

pub(crate) use self::eager::*;
pub(crate) use self::lazy::*;

use super::*;
use crate::builder::HirDeclBuilder;
use husky_hir_eager_expr::HirEagerPatternIdx;
use husky_syn_expr::syndicates::{ParenateParameterSyndicate, SelfValueParameterSyndicate};
use smallvec::SmallVec;

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirParenateParameters {
    Eager(HirEagerParenateParameters),
    Lazy(HirLazyParenateParameters),
}

impl HirParenateParameters {
    #[track_caller]
    pub fn eager(&self) -> &HirEagerParenateParameters {
        match self {
            HirParenateParameters::Eager(slf) => slf,
            HirParenateParameters::Lazy(_) => unreachable!(),
        }
    }
}
