use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum NestedFluffyTerm {
    Ethereal(EtherealTerm),
    /// terms with determined local lifetimes and places, without undetermined arguments
    Solid(SolidTerm),
    Hollow(HollowTerm),
}

impl NestedFluffyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FluffyTermEngine<'b>) -> FluffyTermData<'a>
    where
        'b: 'a,
    {
        let term: FluffyTerm = self.into();
        term.data(engine)
    }
}

impl From<NestedFluffyTerm> for FluffyTerm {
    fn from(term: NestedFluffyTerm) -> Self {
        match term {
            NestedFluffyTerm::Ethereal(term) => term.into(),
            NestedFluffyTerm::Solid(term) => term.into(),
            NestedFluffyTerm::Hollow(term) => term.into(),
        }
    }
}

impl From<FluffyTerm> for NestedFluffyTerm {
    fn from(term: FluffyTerm) -> Self {
        match term {
            FluffyTerm::Solid(term) => NestedFluffyTerm::Solid(term),
            FluffyTerm::Hollow(term) => NestedFluffyTerm::Hollow(term),
            _ => NestedFluffyTerm::Ethereal(term.ethereal().expect("should be ethereal")),
        }
    }
}

impl FluffyTerm {
    pub(crate) fn nested(self) -> NestedFluffyTerm {
        self.into()
    }
}
