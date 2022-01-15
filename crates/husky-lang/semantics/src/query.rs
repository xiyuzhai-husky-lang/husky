mod entity;
mod main;
mod module;

pub use entity::{EntityQueryGroup, EntityQueryGroupStorage};
pub use main::{MainQueryGroup, MainQueryGroupStorage};

use crate::*;

#[salsa::query_group(PackageQueryGroupStorage)]
pub trait PackageQueryGroup: MainQueryGroup + EntityQueryGroup {
    fn package(&self, main_file: file::FileId) -> SemanticResultArc<Package>;
}

fn package(this: &dyn PackageQueryGroup, main_file: file::FileId) -> SemanticResultArc<Package> {
    let scope_id = this.module_from_file_id(main_file)?.scope_id();
    Ok(Arc::new(Package {
        ident: match this.id_to_scope(scope_id).route {
            scope::ScopeRoute::Package(_, ident) => ident,
            _ => panic!(),
        },
        subentities: this.subentities(scope_id)?,
        main: this.main(main_file)?,
    }))
}
