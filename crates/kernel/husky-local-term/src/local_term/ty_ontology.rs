use super::*;

#[derive(Debug, PartialEq, Eq)]
// #[salsa::derive_debug_with_db(db = TermDb)]
pub struct LocalTermTypeOntology {
    path: TypePath,
    arguments: SmallVec<[LocalTerm; 2]>,
}

impl LocalTermTypeOntology {
    pub(super) fn extract_implicit_symbol_dependencies(
        &self,
        unresolved_terms: &UnresolvedTerms,
        dependencies: &mut VecSet<UnresolvedTermIdx>,
    ) {
        let mut f = |term: LocalTerm| {
            unresolved_terms.extract_implicit_symbol_dependencies_aux(term, dependencies)
        };
        self.arguments.iter().copied().for_each(f)
    }

    pub fn path(&self) -> TypePath {
        self.path
    }

    pub fn arguments(&self) -> &[LocalTerm] {
        &self.arguments
    }
}

impl LocalTerm {
    pub fn new_application(
        db: &dyn TermDb,
        local_term_region: &mut LocalTermRegion,
        src_expr_idx: ExprIdx,
        function: impl Into<LocalTerm>,
        argument: impl Into<LocalTerm>,
    ) -> TermResult<Self> {
        match (function.into(), argument.into()) {
            (LocalTerm::Resolved(function), LocalTerm::Resolved(argument)) => {
                Ok(TermApplication::new(db, function, argument)?.into())
            }
            (LocalTerm::Resolved(function), argument) => {
                let expansion = function.application_expansion(db);
                match expansion.function() {
                    TermFunctionReduced::TypeOntology(path) => {
                        let mut arguments: SmallVec<[LocalTerm; 2]> = expansion
                            .arguments(db)
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect();
                        arguments.push(argument);
                        Ok(local_term_region.intern_unresolved_term(
                            src_expr_idx,
                            LocalTermData::TypeOntology(LocalTermTypeOntology { path, arguments }),
                        ))
                    }
                    TermFunctionReduced::Trait(_) => todo!(),
                    TermFunctionReduced::Other(_) => todo!(),
                }
            }
            (LocalTerm::Unresolved(_), LocalTerm::Resolved(_)) => todo!(),
            (LocalTerm::Unresolved(_), LocalTerm::Unresolved(_)) => todo!(),
        }
    }

    pub(crate) fn new_ty_ontology_application(
        db: &dyn TermDb,
        unresolved_terms: &mut UnresolvedTerms,
        src_expr_idx: ExprIdx,
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    ) -> Self {
        let mut resolved_arguments: SmallVec<[Term; 2]> = smallvec![];
        for argument in &arguments {
            match argument {
                LocalTerm::Resolved(argument) => resolved_arguments.push(*argument),
                LocalTerm::Unresolved(_) => break,
            }
        }
        if resolved_arguments.len() == arguments.len() {
            todo!()
        } else {
            unresolved_terms.intern_unresolved_term(
                src_expr_idx,
                LocalTermData::TypeOntology(LocalTermTypeOntology { path, arguments }),
            )
        }
    }
}
