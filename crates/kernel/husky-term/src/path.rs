use crate::*;
use husky_entity_path::EntityPathItd;

impl TermOwned {
    pub fn path(&self) -> EntityPathItd {
        match self {
            TermOwned::Atom(atom) => match atom {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable { variable_variant } => todo!(),
                TermAtom::Entity { path } => *path,
                TermAtom::Category(_) => todo!(),
                TermAtom::Universe(_) => todo!(),
            },
            TermOwned::Curry(_) => todo!(),
            TermOwned::Abstraction(_) => todo!(),
            TermOwned::Application(_) => todo!(),
            TermOwned::Subentity(_) => todo!(),
            TermOwned::TraitImpl(_) => todo!(),
        }
    }
}
