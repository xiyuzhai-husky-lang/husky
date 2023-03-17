use crate::*;
use husky_manifest_ast::ManifestAstDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + ManifestAstDb {}

impl<DB> ManifestDb for DB where DB: DbWithJar<ManifestJar> + ManifestAstDb {}
