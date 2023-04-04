mod dependencies;
mod dev_dependencies;

pub use self::dependencies::*;
pub use self::dev_dependencies::*;

use crate::*;
use husky_corgi_config::HasCorgiConfig;
use husky_manifest_ast::{HasPackageManifestAstSheet, PackageManifestAstSheet};
use husky_word::Word;
