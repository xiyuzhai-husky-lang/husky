use crate::*;

pub trait HirDepsDb: salsa::DbWithJar<HirDepsJar> {}

#[salsa::jar(db = HirDepsDb)]
pub struct HirDepsJar(
    Deps,
    DepPaths0,
    DepPaths1,
    DepPaths2,
    HirLinkageDeps,
    HirValDeps,
);
