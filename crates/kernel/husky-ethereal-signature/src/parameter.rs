mod parenate_parameter;
mod template_parameter;

pub use self::parenate_parameter::*;
pub use self::template_parameter::*;

use crate::*;
use husky_declarative_signature::{
    DeclarativeTemplateParameter, DeclarativeTemplateParameterTemplates,
};
use husky_term_prelude::Variance;
