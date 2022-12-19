use crate::*;
use husky_package_path::PackagePathDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + PackagePathDb {}
