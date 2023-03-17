#![feature(trait_upcasting)]
mod builder;
mod db;
mod dependency;
mod error;
mod menu;
mod sections;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::sections::*;

use self::builder::ManifestAstBuilder;
use husky_text::TextRange;
use husky_toml_ast::*;
use husky_vfs::*;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar(
    ManifestAst,
    manifest_ast,
    ManifestDependencyAst,
    manifest_ast_menu,
);

#[salsa::tracked(db = ManifestAstDb, jar = ManifestAstJar)]
pub struct ManifestAst {
    /// required
    #[return_ref]
    pub package_section: ManifestAstResult<ManifestPackageSectionAst>,
    #[return_ref]
    pub dependencies_section: ManifestAstResult<Option<ManifestDependenciesSectionAst>>,
    #[return_ref]
    pub dev_dependencies_section: ManifestAstResult<Option<ManifestDevDependenciesSectionAst>>,
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
    Ok(build_manifest_ast(db, toml_ast))
}

fn build_manifest_ast(db: &dyn ManifestAstDb, toml_ast: &TomlAstSheet) -> ManifestAst {
    let mut builder = ManifestAstBuilder::new(db, toml_ast, toml_ast.section_visitor());
    let package_section = builder.build_package_section();
    let dependencies_section = builder.build_dependencies_section();
    let dev_dependencies_section = builder.build_dev_dependencies_section();
    ManifestAst::new(
        db,
        package_section,
        dependencies_section,
        dev_dependencies_section,
        builder.finish(),
    )
}
