use crate::*;
use husky_ethereal_term::EtherealTerm;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

impl HirTrait {
    pub fn from_ethereal(trai_term: EtherealTerm, db: &dyn HirTypeDb) -> Self {
        todo!()
    }
}
