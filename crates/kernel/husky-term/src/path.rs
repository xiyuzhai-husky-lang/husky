use crate::*;
use husky_entity_path::EntityPath;

impl<'a> TermRef<'a> {
    pub fn path(&self) -> EntityPath {
        match self {
            TermRef::Atom(atom) => match atom {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable {
                    variable_variant: _,
                } => todo!(),
                TermAtom::Entity { entity_path: path } => *path,
                TermAtom::Category(_) => todo!(),
                TermAtom::Universe(_) => todo!(),
            },
            TermRef::Curry(_) => todo!(),
            TermRef::Abstraction(_) => todo!(),
            TermRef::Application(_) => todo!(),
            TermRef::Subentity(_) => todo!(),
            TermRef::TraitImpl(_) => todo!(),
            TermRef::Null => unreachable!(),
        }
    }
}
