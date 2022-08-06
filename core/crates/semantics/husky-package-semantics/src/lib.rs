mod cfg;
mod query;

pub use cfg::*;
use husky_file::FilePtr;
pub use query::{PackageQueryGroup, PackageQueryGroupStorage};

use std::{path::PathBuf, sync::Arc};

use husky_entity_semantics::*;
use husky_word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub ident: CustomIdentifier,
    pub subentities: Arc<Vec<Arc<EntityDefn>>>,
    pub main_defn: Arc<MainDefn>,
    pub config: Arc<Config>,
}

impl Package {
    pub fn dir(&self) -> PathBuf {
        self.main_defn.file.parent().unwrap().to_owned()
    }

    pub fn crate_entrance(&self) -> FilePtr {
        self.main_defn.file
    }
}
