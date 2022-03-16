use std::sync::Arc;

use crate::{config::Config, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub ident: CustomIdentifier,
    pub subentities: Arc<Vec<Arc<Entity>>>,
    pub main: Arc<Main>,
    pub config: Arc<Config>,
}
