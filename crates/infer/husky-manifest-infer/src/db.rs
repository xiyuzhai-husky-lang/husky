use crate::*;
use husky_vfs::VfsDb;

pub trait ManifestDb: DbWithJar<ManifestJar> + VfsDb {}
