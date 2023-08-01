use crate::{trai::HirTrait, *};
use husky_ethereal_term::EtherealTermDb;

pub trait HirTypeDb: salsa::DbWithJar<HirTypeJar> + EtherealTermDb {}

impl<Db> HirTypeDb for Db where Db: salsa::DbWithJar<HirTypeJar> + EtherealTermDb {}

#[salsa::jar(db = HirTypeDb)]
pub struct HirTypeJar(
    HirTypePathLeading,
    HirTypeTypeAssociatedType,
    HirTypeTraitAssociatedType,
    hir_template_symbol_from_ethereal,
    HirConstSymbol,
    HirTrait,
);
