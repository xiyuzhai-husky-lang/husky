use crate::*;

impl Term {
    /// returns a toolchain except for universe, literals and categories
    pub(crate) fn toolchain(self, db: &dyn TermDb) -> Option<Toolchain> {
        match self {
            Term::Literal(_) => None,
            Term::Symbol(term) => term.toolchain(db),
            Term::Variable(term) => term.toolchain(db),
            Term::EntityPath(path) => Some(path.toolchain(db)),
            Term::Category(_) => todo!(),
            Term::Universe(_) => None,
            Term::Curry(term) => curry_term_toolchain(db, term),
            Term::Ritchie(term) => ritchie_term_toolchain(db, term),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn term_menu(self, db: &dyn TermDb) -> Option<&TermMenu> {
        Some(db.term_menu(self.toolchain(db)?))
    }

    pub(crate) fn leading_ty_path(self, db: &dyn TermDb) -> Option<TypePath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::TypeOntology(path) => Some(path),
            _ => None,
        }
    }

    pub(crate) fn leading_trai_path(self, db: &dyn TermDb) -> Option<TraitPath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::Trait(path) => Some(path),
            _ => None,
        }
    }
}

impl TermSymbol {
    fn toolchain(self, db: &dyn TermDb) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

impl TermVariable {
    fn toolchain(self, db: &dyn TermDb) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn curry_term_toolchain(db: &dyn TermDb, term: TermCurry) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    if let Some(parameter_variable) = term.parameter_variable(db) {
        merger.accept(parameter_variable.toolchain(db))
    }
    merger.accept(term.parameter_ty(db).toolchain(db));
    merger.accept(term.return_ty(db).toolchain(db));
    merger.finish()
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn application_term_toolchain(
    db: &dyn TermDb,
    term: TermApplication,
) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    merger.accept(term.function(db).toolchain(db));
    merger.accept(term.argument(db).toolchain(db));
    merger.finish()
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn ritchie_term_toolchain(db: &dyn TermDb, term: TermRitchie) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    for parameter_liasoned_ty in term.parameter_liasoned_tys(db) {
        merger.accept(parameter_liasoned_ty.ty().toolchain(db))
    }
    merger.accept(term.return_ty(db).toolchain(db));
    merger.finish()
}

#[derive(Debug, Default)]
struct ToolchainMerger {
    toolchain: Option<Toolchain>,
}

impl ToolchainMerger {
    fn accept(&mut self, toolchain: Option<Toolchain>) {
        match (self.toolchain, toolchain) {
            (_, None) => (),
            (None, Some(_)) => self.toolchain = toolchain,
            (Some(self_toolchain), Some(toolchain)) => assert_eq!(self_toolchain, toolchain),
        }
    }

    fn finish(self) -> Option<Toolchain> {
        self.toolchain
    }
}
