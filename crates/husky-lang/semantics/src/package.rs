use std::sync::Arc;

use crate::{config::Config, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub(crate) ident: CustomIdentifier,
    pub(crate) subentities: Arc<Vec<Arc<Entity>>>,
    pub(crate) main: Arc<Main>,
    pub(crate) config: Arc<Config>,
}

impl Package {
    pub fn ident(&self) -> CustomIdentifier {
        self.ident
    }

    pub fn subentities(&self) -> &[Arc<Entity>] {
        &self.subentities
    }

    pub fn main(&self) -> &Main {
        &self.main
    }
}
