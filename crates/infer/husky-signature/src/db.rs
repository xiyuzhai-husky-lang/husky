use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> {}
