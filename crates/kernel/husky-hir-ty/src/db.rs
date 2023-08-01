use crate::{trai::HirTrait, *};
use husky_ethereal_signature::EtherealSignatureDb;

pub trait HirTypeDb: salsa::DbWithJar<HirTypeJar> + EtherealSignatureDb {}

impl<Db> HirTypeDb for Db where Db: salsa::DbWithJar<HirTypeJar> + EtherealSignatureDb {}

#[salsa::jar(db = HirTypeDb)]
pub struct HirTypeJar(
    HirTypePathLeading,
    HirTypeTypeAssociatedType,
    HirTypeTraitAssociatedType,
    hir_template_symbol_from_ethereal,
    HirConstSymbol,
    HirTrait,
    hir_ty_from_ethereal_term_application,
);
