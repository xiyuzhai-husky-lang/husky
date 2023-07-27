use crate::*;

pub trait ValDb: salsa::DbWithJar<ValJar> {}

pub struct ValJar(Val);
