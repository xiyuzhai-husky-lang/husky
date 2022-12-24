mod collector;
mod db;
mod decl;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use sheet::*;

use collector::*;
use husky_entity_path::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    decl_sheet,
    // type
    EnumTypeDecl,
    StructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    TypeAliasDecl,
    // form
    ConstantDecl,
    FeatureDecl,
    FunctionDecl,
    MethodDecl,
    MorphismDecl,
    // trait
    TraitDecl,
);

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> VfsResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}
