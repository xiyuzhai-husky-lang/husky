use husky_ethereal_term::{EtherealTerm, EtherealTermSymbol, EtherealTermSymbolIndexInner};
use husky_term_prelude::TermEntityPath;

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
    pub fn from_ethereal(term: EtherealTerm, db: &dyn HirTypeDb) -> Self {
        match term {
            EtherealTerm::Symbol(symbol) => HirTypeSymbol::from_ethereal(symbol, db).into(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(ty_path) => {
                    HirTypePathLeading::new(db, ty_path, smallvec![]).into()
                }
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            _ => unreachable!("it should be guaranteed that the term is a valid HirType"),
        }
    }
}
