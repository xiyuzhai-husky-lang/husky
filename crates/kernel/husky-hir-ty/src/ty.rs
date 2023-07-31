use husky_ethereal_term::EtherealTerm;

use crate::*;

/// this is much simpler than that in Term, right?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum HirType {
    PathLeading(HirTypePathLeading),
    Symbol(HirTypeSymbol),
    TypeAssociatedType(HirTypeTypeAssociatedType),
    TraitAssociatedType(HirTypeTraitAssociatedType),
    Ritchie(),
}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypeTypeAssociatedType {}

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTypeTraitAssociatedType {}

impl HirType {
    pub fn from_ethereal_term(term: EtherealTerm) -> Self {
        match term {
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::EntityPath(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            _ => unreachable!("it should be guaranteed that the term is a valid HirType"),
        }
    }
}
