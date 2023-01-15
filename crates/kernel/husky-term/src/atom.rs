mod category;
mod entity;
mod literal;
mod universe;
mod variable;

pub use category::*;
pub use entity::*;
use husky_entity_path::EntityPath;
pub use literal::*;
pub use universe::*;
pub use variable::*;

use crate::*;

// HELP: make Debug occupy less lines

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermAtom {
    Literal(TermLiteralData),
    Variable {
        variable_variant: TermVariableVariant,
    },
    Entity {
        entity_path: EntityPath,
    },
    Category(TermCategory),
    Universe(TermUniverse),
}

impl Into<Term> for TermAtom {
    fn into(self) -> Term {
        Term::Atom(self)
    }
}

impl TermAtom {
    // pub(crate) fn ty_term(&self) -> TermCow {
    //     todo!()
    // }

    pub fn universe(&self) -> TermUniverse {
        todo!()
        // if let Some(ty_itd) = self.ty_itd {
        //     match ty_itd.term().deref() {
        //         TermData::Application(app) => match app.m().deref() {
        //             TermData::Atom(m) => match m.variant() {
        //                 TermAtom::Literal(_) => todo!(),
        //                 TermAtom::Variable { variable_variant } => todo!(),
        //                 TermAtom::Entity { path } => todo!(),
        //                 TermAtom::Category(category_kind) => match category_kind {
        //                     TermCategory::Type => todo!(),
        //                     TermCategory::Sort => return app.n().as_universe().unwrap(),
        //                     TermCategory::TermData => todo!(),
        //                 },
        //                 TermAtom::Universe(_) => todo!(),
        //             },
        //             TermData::Curry(_) => todo!(),
        //             TermData::Abstraction(_) => todo!(),
        //             TermData::Application(_) => todo!(),
        //             TermData::Subentity(_) => todo!(),
        //             TermData::TraitImpl(_) => todo!(),
        //         },
        //         _ => (),
        //     }
        //     TermUniverse::zero()
        // } else {
        //     match self.kind {
        //         TermAtom::Literal(_) => todo!(),
        //         TermAtom::Variable {
        //             ref variable_variant,
        //         } => todo!(),
        //         TermAtom::Entity { path } => todo!(),
        //         TermAtom::Category(category_kind) => todo!(),
        //         TermAtom::Universe(_) => todo!(),
        //     }
        // }
    }

    pub(crate) fn new_literal(data: TermLiteralData) -> Self {
        TermAtom::Literal(data)
    }

    pub(crate) fn new_universe(i: u8) -> Self {
        TermAtom::Universe(TermUniverse::new(i))
    }

    pub(crate) fn new_category(category_kind: TermCategory) -> Self {
        TermAtom::Category(category_kind)
    }
}

impl<'a> TermContext<'a> {
    pub fn entity_path_term(&self, entity_path: EntityPath) -> TermResult<Term> {
        todo!()
        // Ok(self.db.it_term(TermAtom::Entity { entity_path }.into()))
    }
}

impl From<i32> for TermAtom {
    fn from(value: i32) -> Self {
        TermAtom::Literal(value.into())
    }
}

impl From<i64> for TermAtom {
    fn from(value: i64) -> Self {
        TermAtom::Literal(value.into())
    }
}
