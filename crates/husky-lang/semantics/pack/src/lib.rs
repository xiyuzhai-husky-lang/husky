mod cfg;
mod query;

pub use cfg::*;
pub use query::{PackageQueryGroup, PackageQueryGroupStorage};

use std::sync::Arc;

use semantics_entity::*;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub ident: CustomIdentifier,
    pub subentities: Arc<Vec<Arc<Entity>>>,
    pub main: Arc<Main>,
    pub config: Arc<Config>,
}
