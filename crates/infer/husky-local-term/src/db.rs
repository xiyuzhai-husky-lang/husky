use crate::*;

pub trait LocalTermDb: salsa::DbWithJar<LocalTermJar> {}

impl<T> LocalTermDb for T where T: salsa::DbWithJar<LocalTermJar> {}
