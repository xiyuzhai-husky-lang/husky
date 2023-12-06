pub mod final_destination;
pub mod toolchain;

use crate::*;

impl EtherealTerm {
    pub fn leading_ty_path(self, db: &::salsa::Db) -> Option<TypePath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::TypeOntology(path) => Some(path),
            _ => None,
        }
    }

    pub fn leading_trai_path(self, db: &::salsa::Db) -> Option<TraitPath> {
        match self.application_expansion(db).function() {
            TermFunctionReduced::Trait(path) => Some(path),
            _ => None,
        }
    }

    /// see `self` as the type of another term, return the type expectation for that term
    pub fn ty_expectation(self, db: &::salsa::Db) -> EtherealTermResult<TermTypeExpectation> {
        Ok(match self.application_expansion(db).function() {
            TermFunctionReduced::TypeOntology(path) => {
                TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path)
            }
            _ => TermTypeExpectation::Any,
        })
    }

    pub fn synthesize_function_application_expr_ty(
        db: &::salsa::Db,
        variance: Variance,
        parameter_symbol: Option<EtherealTerm>,
        parameter_ty: EtherealTerm,
        return_ty: EtherealTerm,
        argument_ty: EtherealTerm,
        shift: i8,
    ) -> EtherealTermResult<EtherealTerm> {
        if shift == 0 {
            if parameter_symbol.is_some() {
                todo!()
            }
            return Ok(return_ty);
        }
        if parameter_symbol.is_some() {
            todo!()
        }
        match argument_ty {
            EtherealTerm::Curry(argument_ty) => {
                let expr_ty = Self::synthesize_function_application_expr_ty(
                    db,
                    variance,
                    parameter_symbol,
                    parameter_ty,
                    return_ty,
                    argument_ty.return_ty(db),
                    shift - 1,
                )?;
                Ok(EtherealTermCurry::new(
                    db,
                    argument_ty.curry_kind(db),
                    argument_ty.variance(db),
                    None,
                    argument_ty.parameter_ty(db),
                    expr_ty,
                )
                .into())
            }
            _ => unreachable!(),
        }
    }

    pub fn new_ty_ontology(
        db: &::salsa::Db,
        path: TypePath,
        arguments: impl Iterator<Item = EtherealTerm>,
    ) -> EtherealTermResult<Self> {
        let mut term: Self = TermEntityPath::TypeOntology(path).into();
        for argument in arguments {
            term = EtherealTermApplication::new(db, term, argument)?
        }
        Ok(term)
    }
}

impl EtherealTermSymbol {
    fn toolchain(self, db: &::salsa::Db) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

impl EtherealTermRune {
    fn toolchain(self, db: &::salsa::Db) -> Option<Toolchain> {
        self.ty(db).toolchain(db)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_curry_toolchain(
    db: &::salsa::Db,
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
pub(crate) fn ethereal_term_application_toolchain(
    db: &::salsa::Db,
    term: EtherealTermApplication,
) -> Option<Toolchain> {
    let mut merger = ToolchainMerger::default();
    merger.accept(term.function(db).toolchain(db));
    merger.accept(term.argument(db).toolchain(db));
    merger.finish()
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_ritchie_toolchain(
    db: &::salsa::Db,
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
