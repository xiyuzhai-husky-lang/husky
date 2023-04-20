use crate::*;

impl EtherealTerm {
    /// returns a toolchain except for universe, literals and categories
    pub(crate) fn toolchain(self, db: &dyn EtherealTermDb) -> Option<Toolchain> {
        match self {
            EtherealTerm::Literal(_) => None,
            EtherealTerm::Symbol(term) => term.toolchain(db),
            EtherealTerm::Variable(term) => term.toolchain(db),
            EtherealTerm::EntityPath(path) => Some(path.toolchain(db)),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => None,
            EtherealTerm::Curry(term) => curry_term_toolchain(db, term),
            EtherealTerm::Ritchie(term) => ritchie_term_toolchain(db, term),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub(crate) fn term_menu(self, db: &dyn EtherealTermDb) -> Option<&TermMenu> {
        Some(db.term_menu(self.toolchain(db)?))
    }

    pub(crate) fn leading_ty_path(self, db: &dyn EtherealTermDb) -> Option<TypePath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::TypeOntology(path) => Some(path),
            _ => None,
        }
    }

    pub(crate) fn leading_trai_path(self, db: &dyn EtherealTermDb) -> Option<TraitPath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::Trait(path) => Some(path),
            _ => None,
        }
    }
}

impl EtherealTermSymbol {
    fn toolchain(self, db: &dyn EtherealTermDb) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

impl EtherealTermVariable {
    fn toolchain(self, db: &dyn EtherealTermDb) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn curry_term_toolchain(
    db: &dyn EtherealTermDb,
    term: EtherealTermCurry,
) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    if let Some(parameter_variable) = term.parameter_variable(db) {
        merger.accept(parameter_variable.toolchain(db))
    }
    merger.accept(term.parameter_ty(db).toolchain(db));
    merger.accept(term.return_ty(db).toolchain(db));
    merger.finish()
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn application_term_toolchain(
    db: &dyn EtherealTermDb,
    term: EtherealTermApplication,
) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    merger.accept(term.function(db).toolchain(db));
    merger.accept(term.argument(db).toolchain(db));
    merger.finish()
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ritchie_term_toolchain(
    db: &dyn EtherealTermDb,
    term: EtherealTermRitchie,
) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    for parameter_contracted_ty in term.parameter_contracted_tys(db) {
        merger.accept(parameter_contracted_ty.ty().toolchain(db))
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
