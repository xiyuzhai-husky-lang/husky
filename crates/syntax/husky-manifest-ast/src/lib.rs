#![feature(trait_upcasting)]
mod db;
mod dependency;
mod error;
mod menu;
mod parser;
mod sections;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::sections::*;

use self::parser::ManifestAstParser;
use husky_text::TextRange;
use husky_toml_ast::*;
use husky_vfs::*;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar(
    PackageManifestAstSheet,
    package_manifest_ast_sheet,
    manifest_ast_menu,
);

#[salsa::tracked(db = ManifestAstDb, jar = ManifestAstJar)]
pub struct PackageManifestAstSheet {
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

pub trait HasPackageManifestAstSheet: Copy {
    fn manifest_ast_sheet(self, db: &dyn ManifestAstDb) -> VfsResult<PackageManifestAstSheet>;
}

impl HasPackageManifestAstSheet for PackagePath {
    fn manifest_ast_sheet(self, db: &dyn ManifestAstDb) -> VfsResult<PackageManifestAstSheet> {
        package_manifest_ast_sheet(db, self)
    }
}

#[salsa::tracked(jar = ManifestAstJar)]
fn package_manifest_ast_sheet(
    db: &dyn ManifestAstDb,
    path: PackagePath,
) -> VfsResult<PackageManifestAstSheet> {
    let toml_ast_sheet = db.package_manifest_toml_ast_sheet(path)?;
    Ok(build_package_manifest_ast_sheet(db, toml_ast_sheet))
}

fn build_package_manifest_ast_sheet(
    db: &dyn ManifestAstDb,
    toml_ast_sheet: &TomlAstSheet,
) -> PackageManifestAstSheet {
    todo!()
    // let mut parser: ManifestAstParser<TomlTable> =
    //     ManifestAstParser::new(db, toml_ast_sheet, toml_ast_sheet.root_visitor());
    // let mut errors = vec![];
    // let package_section = parser.parse_package_section(&mut errors);
    // let dependencies_section = parser.parse_dependencies_section(db, &mut errors);
    // let dev_dependencies_section = parser.parse_dev_dependencies_section(&mut errors);
    // PackageManifestAstSheet::new(
    //     db,
    //     package_section,
    //     dependencies_section,
    //     dev_dependencies_section,
    //     errors,
    // )
}
