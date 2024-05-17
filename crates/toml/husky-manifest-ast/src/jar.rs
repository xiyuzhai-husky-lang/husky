use crate::*;

#[salsa::jar]
pub struct ManifestAstJar(package_manifest_ast_sheet_aux, manifest_ast_menu);
