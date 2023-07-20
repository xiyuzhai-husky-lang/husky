mod generic;
mod specific;

pub use self::generic::*;
pub use self::specific::*;

use crate::*;
use husky_declarative_signature::{
    DeclarativeGenericParameter, DeclarativeGenericParameterTemplates,
};
use husky_term_prelude::Variance;
