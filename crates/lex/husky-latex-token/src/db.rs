use crate::*;
use husky_vfs::*;
use salsa::DbWithJar;

pub trait LaTexTokenDb: DbWithJar<LaTexTokenJar> + VfsDb {}

impl<T> LaTexTokenDb for T where T: DbWithJar<LaTexTokenJar> + VfsDb + VfsDb {}
