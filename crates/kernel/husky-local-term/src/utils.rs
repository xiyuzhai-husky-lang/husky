use super::*;

impl LocalTerm {
    pub(crate) fn curry_destination(
        self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTerm {
        match self.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::Literal(_)
            | LocalTermPattern::TypeOntology { .. }
            | LocalTermPattern::ImplicitSymbol(_, _)
            | LocalTermPattern::Category(_) => self,
            LocalTermPattern::Curry { .. } => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }

    /// this term as ty, what's its final destination?
    pub(crate) fn final_destination_inner(
        self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        match self.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology { .. } => FinalDestination::TypeOntology,
            LocalTermPattern::Curry { .. } => todo!(),
            LocalTermPattern::ImplicitSymbol(kind, idx) => match kind {
                ImplicitSymbolKind::ImplicitLifetime => todo!(),
                ImplicitSymbolKind::ExprEvalLifetime => todo!(),
                ImplicitSymbolKind::UnspecifiedIntegerType
                | ImplicitSymbolKind::UnspecifiedFloatType
                | ImplicitSymbolKind::ImplicitType => FinalDestination::TypeOntology,
            },
            LocalTermPattern::Category(_) => FinalDestination::Sort,
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }
}

fn curry_destination(db: &dyn TermDb, term: Term) -> Term {
    match term {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Hole(_) => todo!(),
        Term::EntityPath(path) => match path {
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
        Term::Category(_) => term,
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => term,
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
        Term::Place(_) => todo!(),
    }
}
