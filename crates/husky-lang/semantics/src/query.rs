mod config;
mod entity;
pub(crate) mod infer;
mod main;
mod module;

use std::sync::Arc;

pub use config::{ConfigQueryGroup, ConfigQueryGroupStorage};
pub use entity::{EntityQueryGroup, EntityQueryGroupStorage};
pub use infer::{InferQueryGroup, InferQueryGroupStorage};
use interner::InternId;
pub use main::{MainQueryGroup, MainQueryGroupStorage};

use crate::{Package, SemanticResultArc};

#[salsa::query_group(PackageQueryGroupStorage)]
pub trait PackageQueryGroup: MainQueryGroup + EntityQueryGroup + ConfigQueryGroup {
    fn package(&self, main_file: file::FileId) -> SemanticResultArc<Package>;
}

fn package(this: &dyn PackageQueryGroup, main_file: file::FileId) -> SemanticResultArc<Package> {
    let scope = this.module_from_file_id(main_file)?.scope();
    Ok(Arc::new(Package {
        ident: match scope.route {
            scope::ScopeRoute::Package { ident, .. } => ident,
            _ => panic!(),
        },
        subentities: this.subentities(scope)?,
        main: this.main(main_file)?,
        config: this.config(main_file)?,
    }))
}
