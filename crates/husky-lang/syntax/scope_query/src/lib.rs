mod error;
mod query;
mod subscope;

pub use error::{def::ScopeDefError, ScopeError, ScopeResult, ScopeResultArc};
pub use query::{
    ModuleFromFileError, PackageOrModule, ScopeQueryGroup, ScopeQueryGroupStorage,
    ScopeSalsaQueryGroup,
};

use subscope::*;
