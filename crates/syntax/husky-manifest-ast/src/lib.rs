#![feature(result_flattening)]
#![feature(trait_upcasting)]
mod db;
mod dependency;
mod error;
mod menu;
mod sections;
mod transformer;

pub use self::db::*;
pub use self::dependency::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::sections::*;

use self::transformer::*;
use husky_text_protocol::range::TextRange;
use husky_toml_ast::*;
use husky_vfs::*;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar(package_manifest_ast_sheet_aux, manifest_ast_menu);

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ManifestAstDb)]
pub struct PackageManifestAstSheet {
    package_section: ManifestAstResult<ManifestPackageSectionAst>,
    dependencies_section: Option<ManifestAstResult<ManifestDependenciesSectionAst>>,
    dev_dependencies_section: Option<ManifestAstResult<ManifestDevDependenciesSectionAst>>,
    errors: Vec<ManifestAstError>,
}

impl PackageManifestAstSheet {
    pub fn package_section(&self) -> Result<&ManifestPackageSectionAst, &ManifestAstError> {
        self.package_section.as_ref()
    }

    pub fn dependencies_section(
        &self,
    ) -> Option<ManifestAstResultRef<&ManifestDependenciesSectionAst>> {
        self.dependencies_section.as_ref().map(|r| r.as_ref())
    }

    pub fn dev_dependencies_section(
        &self,
    ) -> Option<ManifestAstResultRef<&ManifestDevDependenciesSectionAst>> {
        self.dev_dependencies_section.as_ref().map(|r| r.as_ref())
    }

    pub fn errors(&self) -> &[ManifestAstError] {
        self.errors.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ManifestExprVariant {
    Equals { name: String },
}

pub trait HasPackageManifestAstSheet: Copy {
    fn manifest_ast_sheet(self, db: &dyn ManifestAstDb) -> VfsResult<&PackageManifestAstSheet>;
}

impl HasPackageManifestAstSheet for PackagePath {
    fn manifest_ast_sheet(self, db: &dyn ManifestAstDb) -> VfsResult<&PackageManifestAstSheet> {
        package_manifest_ast_sheet(db, self)
    }
}

fn package_manifest_ast_sheet(
    db: &dyn ManifestAstDb,
    path: PackagePath,
) -> VfsResult<&PackageManifestAstSheet> {
    package_manifest_ast_sheet_aux(db, path)
        .as_ref()
        .map_err(|e| e.clone())
}

#[salsa::tracked(jar = ManifestAstJar, return_ref)]
fn package_manifest_ast_sheet_aux(
    db: &dyn ManifestAstDb,
    path: PackagePath,
) -> VfsResult<PackageManifestAstSheet> {
    let mut errors = vec![];
    let mut transformer: ManifestAstTransformer<TomlTable> =
        ManifestAstTransformer::new_root_expected(
            db,
            path.manifest_path(db)?.path(),
            manifest_ast_menu(db),
            &mut errors,
        )?;
    Ok(PackageManifestAstSheet {
        package_section: transformer
            .transform_normal_section()
            .ok_or(OriginalManifestAstError::MissingPackageSection.into())
            .flatten(),
        dependencies_section: transformer.transform_normal_section(),
        dev_dependencies_section: transformer.transform_normal_section(),
        errors: transformer.finish(),
    })
}
