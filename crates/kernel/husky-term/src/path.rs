use crate::*;
use husky_entity_path::EntityPathItd;

impl Term {
    pub fn path(&self) -> EntityPathItd {
        match self {
            Term::Atom(atom) => match atom.variant() {
                TermAtomVariant::Literal(_) => todo!(),
                TermAtomVariant::Variable { variable_variant } => todo!(),
                TermAtomVariant::Entity { path } => *path,
                TermAtomVariant::Category(_) => todo!(),
                TermAtomVariant::Universe(_) => todo!(),
            },
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }
}
