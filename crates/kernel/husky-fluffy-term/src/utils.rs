use super::*;

impl FluffyTerm {
    pub(crate) fn curry_destination(
        self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FluffyTerm {
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_)
            | FluffyTermData::TypeOntology { .. }
            | FluffyTermData::Hole(_, _)
            | FluffyTermData::Category(_) => self,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }

    /// this term as ty, what's its final destination?
    pub(crate) fn final_destination_inner(
        self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        match self.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology { .. } => FinalDestination::TypeOntology,
            FluffyTermData::Curry { .. } => todo!(),
            FluffyTermData::Hole(kind, idx) => match kind {
                HoleKind::UnspecifiedIntegerType
                | HoleKind::UnspecifiedFloatType
                | HoleKind::ImplicitType => FinalDestination::TypeOntology,
            },
            FluffyTermData::Category(_) => FinalDestination::Sort,
            FluffyTermData::Ritchie { .. } => todo!(),
            FluffyTermData::PlaceTypeOntology { .. } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}

fn curry_destination(db: &dyn EtherealTermDb, term: EtherealTerm) -> EtherealTerm {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(_) => todo!(),
        EtherealTerm::Placeholder(_) => todo!(),
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_)
            | TermEntityPath::TypeOntology(_)
            | TermEntityPath::TypeConstructor(_) => term,
        },
        // EntityPath::Module(_) => todo!(),
        // EntityPath::ModuleItem(path) => match path {
        //     ModuleItemPath::Type(path) => resolved_term,
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // EntityPath::AssociatedItem(_) => todo!(),
        // EntityPath::Variant(_) => todo!(),
        EtherealTerm::Category(_) => term,
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(_) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(_) => term,
        EtherealTerm::Subentity(_) => todo!(),
        EtherealTerm::AsTraitSubentity(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}
