mod builder;
mod db;
mod dependency;
mod error;
mod sections;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::sections::*;

use husky_text::TextRange;
use husky_vfs::*;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar(ManifestAst, manifest_ast);

#[salsa::tracked(db = ManifestAstDb, jar = ManifestAstJar)]
pub struct ManifestAst {
    #[return_ref]
    pub package_options: ManifestPackageSectionAst,
    #[return_ref]
    pub dependencies: Vec<ManifestDependenyAst>,
    #[return_ref]
    pub dev_dependencies: Vec<ManifestDependenyAst>,
    #[return_ref]
    pub errors: Vec<ManifestAstError>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ManifestExprVariant {
    Equals { name: String },
}

pub trait HasManifestAst: Copy {
    fn manifest_ast(self, db: &dyn ManifestAstDb) -> VfsResult<ManifestAst>;
}

impl HasManifestAst for PackagePath {
    fn manifest_ast(self, db: &dyn ManifestAstDb) -> VfsResult<ManifestAst> {
        manifest_ast(db, self)
    }
}

#[salsa::tracked(jar = ManifestAstJar)]
fn manifest_ast(db: &dyn ManifestAstDb, path: PackagePath) -> VfsResult<ManifestAst> {
    let toml_ast = db.package_manifest_toml_ast(path)?;
    todo!()
}
