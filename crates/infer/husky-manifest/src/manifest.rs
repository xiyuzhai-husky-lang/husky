mod dependency;

pub use self::dependency::*;

use crate::*;
use husky_word::Word;

#[salsa::tracked(db = ManifestDb, jar = ManifestJar)]
pub struct PackageManifest {
    #[return_ref]
    dependencies: Vec<PackageDependency>,
    #[return_ref]
    dev_dependencies: Vec<PackageDependency>,
}
