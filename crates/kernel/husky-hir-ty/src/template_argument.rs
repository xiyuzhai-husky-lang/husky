use crate::*;
use husky_ethereal_term::EtherealTerm;

pub type HirTemplateArguments = smallvec::SmallVec<[HirTemplateArgument; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateArgument {
    /// `Vacant` is used to repr abstract types
    ///
    /// say a list of any element type
    ///
    /// It doesn't mean two elements in the list can be of different type
    ///
    /// It just means that the type is capable of representing any list,
    /// saving the need to recompile.
    ///
    /// It should be noted that phantom template parameter should only accept vacant parameter.
    Vacant,
    Type(HirType),
    TypeConstant(HirConstant),
}

impl HirTemplateArgument {
    pub(crate) fn from_ethereal(argument: EtherealTerm, db: &dyn HirTypeDb) -> Self {
        match argument {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => todo!(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}
