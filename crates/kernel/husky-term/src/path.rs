use crate::*;
use husky_entity_path::EntityPathItd;

impl<'a> TermRef<'a> {
    pub fn path(&self) -> EntityPathItd {
        match self {
            TermRef::Atom(atom) => match atom {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable {
                    variable_variant: _,
                } => todo!(),
                TermAtom::Entity { path } => *path,
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
