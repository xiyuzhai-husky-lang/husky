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
            LocalTerm::Resolved(resolved_term) => match resolved_term.term() {
                Term::Literal(_) => todo!(),
                Term::Symbol(_) => todo!(),
                Term::Entity(path) => match path {
                    EntityPath::Module(_) => todo!(),
                    EntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(ty_path) => FinalDestination::TypePath(ty_path),
                        ModuleItemPath::Trait(_) => todo!(),
                        ModuleItemPath::Form(_) => todo!(),
                    },
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::Variant(_) => todo!(),
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
                        Term::Entity(path) => match path {
                            EntityPath::Module(_) => todo!(),
                            EntityPath::ModuleItem(path) => match path {
                                ModuleItemPath::Type(ty_path) => {
                                    FinalDestination::TypePath(ty_path)
                                }
                                ModuleItemPath::Trait(_) => todo!(),
                                ModuleItemPath::Form(_) => todo!(),
                            },
                            EntityPath::AssociatedItem(_) => todo!(),
                            EntityPath::Variant(_) => todo!(),
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

fn curry_destination(db: &dyn ExprTypeDb, resolved_term: ReducedTerm) -> ReducedTerm {
    match resolved_term.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(path) => match path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => resolved_term,
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        },
        Term::Category(_) => resolved_term,
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => resolved_term,
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}
