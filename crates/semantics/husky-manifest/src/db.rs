use crate::*;
use husky_corgi_config::CorgiConfigDb;
use husky_manifest_ast::ManifestAstDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + ManifestAstDb + CorgiConfigDb {}

implManifestDb for DB where DB: DbWithJar<ManifestJar> + ManifestAstDb + CorgiConfigDb {}
