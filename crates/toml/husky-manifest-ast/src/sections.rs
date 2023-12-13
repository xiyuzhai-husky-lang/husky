mod dependencies;
mod dev_dependencies;
mod package;

pub(crate) use self::dependencies::*;
pub(crate) use self::dev_dependencies::*;
pub(crate) use self::package::*;

use husky_coword::Coword;



use crate::*;
