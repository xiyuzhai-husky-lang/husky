mod db;
mod dependency;
mod error;

pub use db::*;
pub use error::*;

use dependency::*;
use husky_text::TextRange;
use salsa::DbWithJar;

#[salsa::jar(db = ManifestDb)]
pub struct ManifestJar(unchecked_package_dependencies, package_dependencies);

pub struct PackageManifest {}
