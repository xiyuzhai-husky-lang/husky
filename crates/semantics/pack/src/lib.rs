mod cfg;
mod query;

pub use cfg::*;
pub use query::{PackQueryGroup, PackQueryGroupStorage};

use std::sync::Arc;

use semantics_entity::*;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pack {
    pub ident: CustomIdentifier,
    pub subentity_defns: Arc<Vec<Arc<EntityDefn>>>,
    pub main_defn: Arc<MainDefn>,
    pub config: Arc<Config>,
}
