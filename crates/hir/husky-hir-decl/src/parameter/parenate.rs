pub mod eager;
pub mod lazy;

pub(crate) use self::eager::*;
pub(crate) use self::lazy::*;

use super::*;
use crate::{builder::HirDeclBuilder, db::HirDeclDb};
use husky_hir_eager_expr::HirEagerPatternExprIdx;
use husky_syn_expr::{ParenateParameterSyndicate, SelfValueParameterSyndicate};
use smallvec::SmallVec;
