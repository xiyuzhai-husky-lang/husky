use crate::{deps::ValDeps, Val};

#[salsa::jar(db = ValDb)]
pub struct ValJar(Val, ValDeps);
