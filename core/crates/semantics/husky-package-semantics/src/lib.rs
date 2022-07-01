mod cfg;
mod query;

pub use cfg::*;
use file::FilePtr;
pub use query::{PackageQueryGroup, PackageQueryGroupStorage};

use std::sync::Arc;

use husky_entity_semantics::*;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub ident: CustomIdentifier,
    pub subentities: Arc<Vec<Arc<EntityDefn>>>,
    pub main_defn: Arc<MainDefn>,
    pub config: Arc<Config>,
}
