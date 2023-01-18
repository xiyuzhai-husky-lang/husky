use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> + DeclDb + TermDb {}

impl<Db> SignatureDb for Db where Db: salsa::DbWithJar<SignatureJar> + DeclDb + TermDb {}
