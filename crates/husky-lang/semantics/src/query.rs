mod config;
mod entity;
pub(crate) mod infer;
mod main;
mod module;

use std::sync::Arc;

pub use config::{ConfigQueryGroup, ConfigQueryGroupStorage};
pub use entity::{EntityQueryGroup, EntityQueryGroupStorage};
pub use infer::{InferQueryGroup, InferQueryGroupStorage};
pub use main::{MainQueryGroup, MainQueryGroupStorage};
use unique_allocator::UniqueAllocatorPtr;

use crate::{Package, SemanticResultArc};

#[salsa::query_group(PackageQueryGroupStorage)]
pub trait PackageQueryGroup: MainQueryGroup + EntityQueryGroup + ConfigQueryGroup {
    fn package(&self, main_file: file::FilePtr) -> SemanticResultArc<Package>;
}

fn package(this: &dyn PackageQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Package> {
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
