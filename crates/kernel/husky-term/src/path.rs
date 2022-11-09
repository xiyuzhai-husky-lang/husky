use crate::*;
use husky_entity_path::EntityPathItd;

impl Term {
    pub fn path(&self) -> EntityPathItd {
        match self {
            Term::Atom(atom) => match atom {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable { variable_variant } => todo!(),
                TermAtom::Entity { path } => *path,
                TermAtom::Category(_) => todo!(),
                TermAtom::Universe(_) => todo!(),
            },
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }
}
