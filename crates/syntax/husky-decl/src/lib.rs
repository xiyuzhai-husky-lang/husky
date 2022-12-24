mod collector;
mod db;
mod decl_data;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl_data::*;
pub use sheet::*;

use collector::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    // type
    TypeAliasDecl,
    EnumTypeDecl,
    InductiveTypeDecl,
    StructTypeDecl,
    StructureTypeDecl,
    // form
    Decl,
    decl_sheet,
    ConstantDecl,
    FeatureDecl,
    FunctionDecl,
    MethodDecl,
    MorphismDecl,
    // trait
    TraitDecl,
);

#[salsa::tracked(jar = DeclJar)]
pub struct Decl {
    #[return_ref]
    data: DeclData,
}

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> VfsResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}
