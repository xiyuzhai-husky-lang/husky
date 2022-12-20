use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermApplication {
    m: Term,
    n: Term,
}

impl Into<TermData> for TermApplication {
    fn into(self) -> TermData {
        TermData::Application(self)
    }
}

impl TermApplication {
    pub fn ty_itd(&self) -> Option<Term> {
        // TODO: delete this
        None
    }

    pub fn m(&self) -> Term {
        self.m
    }

    pub fn n(&self) -> Term {
        self.n
    }

    // pub(crate) fn ty_term(&self) -> TermCow {
    //     if let Some(ty_itd) = self.ty_itd {
    //         ty_itd.term().into()
    //     } else {
    //         match self.m.deref() {
    //             TermData::Atom(a) => match a.variant() {
    //                 TermAtomVariant::Literal(_) => todo!(),
    //                 TermAtomVariant::Variable { variable_variant } => todo!(),
    //                 TermAtomVariant::Entity {} => todo!(),
    //                 TermAtomVariant::Category { category_kind } => {
    //                     self.n.as_universe().unwrap().next().into()
    //                 },
    //                 TermAtomVariant::Universe(_) => todo!(),
    //             },
    //             TermData::Curry(_) => todo!(),
    //             TermData::Abstraction(_) => todo!(),
    //             TermData::Application(_) => todo!(),
    //         }
    //     }
    // }

    pub fn new(m: Term, n: Term) -> TermResult<Self> {
        // ad hoc
        // TODO: add type checking
        Ok(Self { m, n })
        // if m.ty_itd().is_none() {
        //     match m.deref() {
        //         TermData::Atom(a) => match a {
        //             TermAtom::Category(category_kind) => match n.deref() {
        //                 TermData::Atom(b) => match b {
        //                     TermAtom::Literal(_) => todo!(),
        //                     TermAtom::Variable { variable_variant } => todo!(),
        //                     TermAtom::Entity { .. } => todo!(),
        //                     TermAtom::Category(category_kind) => todo!(),
        //                     TermAtom::Universe(_) => Ok(Self { m, n, ty_itd: None }),
        //                 },
        //                 TermData::Curry(_) => todo!(),
        //                 TermData::Abstraction(_) => todo!(),
        //                 TermData::Application(_) => todo!(),
        //                 TermData::Subentity(_) => todo!(),
        //                 TermData::TraitImpl(_) => todo!(),
        //             },
        //             TermAtom::Universe(_) => todo!(),
        //             _ => unreachable!(),
        //         },
        //         _ => unreachable!(),
        //     }
        // } else {
        //     todo!()
        // }
    }
}

impl<'a> TermContext<'a> {
    pub(crate) fn sort(&self, _universe: TermUniverse) -> Term {
        todo!()
        // self.it_term(
        //     TermApplication {
        //         m: self.it_term(TermCategory::Sort.into()),
        //         n: self.it_term(universe.into()),
        //     }
        //     .into(),
        // )
    }
}

impl std::fmt::Display for TermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
