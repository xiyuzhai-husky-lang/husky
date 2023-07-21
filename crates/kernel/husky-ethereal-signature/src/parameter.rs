mod generic;
mod parenic;

pub use self::generic::*;
pub use self::parenic::*;

use crate::*;
use husky_declarative_signature::{
    DeclarativeGenericParameter, DeclarativeGenericParameterTemplates,
};
use husky_term_prelude::Variance;
