use crate::*;

pub trait HirTypeDb: salsa::DbWithJar<HirTypeJar> {}

#[salsa::jar(db = HirTypeDb)]
pub struct HirTypeJar(
    HirTypePathLeading,
    HirTypeTypeAssociatedType,
    HirTypeTraitAssociatedType,
    HirConstantSymbol,
);
