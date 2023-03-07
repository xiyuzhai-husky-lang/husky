use super::*;

impl LocalTerm {
    pub(crate) fn curry_destination(
        self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTerm {
        match self {
            LocalTerm::Resolved(resolved_term) => curry_destination(db, resolved_term).into(),
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    pub(crate) fn final_destination(
        self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        match self.curry_destination(db, unresolved_terms) {
            LocalTerm::Resolved(resolved_term) => match resolved_term {
                Term::Literal(_) => todo!(),
                Term::Symbol(_) => todo!(),
                Term::EntityPath(path) => match path {
                    TermEntityPath::Form(_) => todo!(),
                    TermEntityPath::Trait(_) => todo!(),
                    TermEntityPath::TypeOntology(path) => FinalDestination::TypeOntologyPath(path),
                    TermEntityPath::TypeConstructor(path) => {
                        FinalDestination::TypeConstructorPath(path)
                    }
                },
                Term::Category(_) => FinalDestination::Sort,
                Term::Universe(_) => todo!(),
                Term::Curry(_) => todo!(),
                Term::Ritchie(_) => todo!(),
                Term::Abstraction(_) => todo!(),
                Term::Application(_) => {
                    let expansion = db.term_application_expansion(resolved_term);
                    match expansion.f() {
                        Term::Literal(_) => todo!(),
                        Term::Symbol(_) => todo!(),
                        Term::EntityPath(path) => match path {
                            TermEntityPath::Form(_) => todo!(),
                            TermEntityPath::Trait(_) => todo!(),
                            TermEntityPath::TypeOntology(ty_path) => {
                                FinalDestination::TypeOntologyPath(ty_path)
                            }
                            TermEntityPath::TypeConstructor(_) => todo!(),
                        },
                        Term::Category(_) => todo!(),
                        Term::Universe(_) => todo!(),
                        Term::Curry(_) => todo!(),
                        Term::Ritchie(_) => todo!(),
                        Term::Abstraction(_) => todo!(),
                        Term::Application(_) => todo!(),
                        Term::Subentity(_) => todo!(),
                        Term::AsTraitSubentity(_) => todo!(),
                        Term::TraitConstraint(_) => todo!(),
                    }
                }
                Term::Subentity(_) => todo!(),
                Term::AsTraitSubentity(_) => todo!(),
                Term::TraitConstraint(_) => todo!(),
            },
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}

fn curry_destination(db: &dyn ExprTypeDb, term: Term) -> Term {
    match term {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
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
    }
}
