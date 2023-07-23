use super::*;

impl FluffyTerm {
    #[inline(always)]
    pub fn new_application(
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        function: impl Into<NestedFluffyTerm>,
        argument: impl Into<NestedFluffyTerm>,
    ) -> EtherealTermResult<Self> {
        let db = engine.db();
        let function = function.into();
        let argument = argument.into();
        match (function, argument) {
            (NestedFluffyTerm::Ethereal(function), NestedFluffyTerm::Ethereal(argument)) => {
                Ok(EtherealTermApplication::new(db, function, argument)?.into())
            }
            (
                NestedFluffyTerm::Ethereal(_) | NestedFluffyTerm::Solid(_),
                NestedFluffyTerm::Ethereal(_) | NestedFluffyTerm::Solid(_),
            ) => {
                todo!()
            }
            _ => {
                let data = match function.data(engine) {
                    FluffyTermData::Literal(_) => todo!(),
                    FluffyTermData::TypeOntology {
                        ty_path: path,
                        refined_ty_path: refined_path,
                        arguments,
                        ..
                    } => {
                        let mut arguments = arguments.to_smallvec();
                        arguments.push(argument.into());
                        HollowTermData::TypeOntology {
                            path,
                            refined_path,
                            arguments,
                        }
                    }
                    FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
                    FluffyTermData::Curry {
                        curry_kind,
                        variance,
                        parameter_variable,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term
                    } => todo!(),
                    FluffyTermData::Hole(_, _) => todo!(),
                    FluffyTermData::Category(_) => todo!(),
                    FluffyTermData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                        ..
                    } => todo!(),
                    FluffyTermData::HoleAtPlace {
                        place,
                        hole_kind,
                        hole,
                    } => todo!(),
                    FluffyTermData::Symbol { .. } => todo!(),
                    FluffyTermData::SymbolAtPlace { .. } => todo!(),
                    FluffyTermData::Variable { ty } => todo!(),
                    FluffyTermData::TypeVariant { path } => todo!(),
                };
                Ok(HollowTerm::new(engine, data).into())
            }
            // (NestedFluffyTerm::Ethereal(function), argument) => {
            //     let expansion = function.application_expansion(db);
            //     match expansion.function() {
            //         TermFunctionReduced::TypeOntology(path) => {
            //             let mut arguments: SmallVec<[FluffyTerm; 2]> = expansion
            //                 .arguments(db)
            //                 .iter()
            //                 .copied()
            //                 .map(Into::into)
            //                 .collect();
            //             arguments.push(argument.into());
            //             Ok(engine.fluffy_term_region_mut().intern_unresolved_term(
            //                 src,
            //                 FluffyTermData::TypeOntology(FluffyTermTypeOntology {
            //                     path,
            //                     arguments,
            //                 }),
            //             ))
            //         }
            //         TermFunctionReduced::Trait(_) => todo!(),
            //         TermFunctionReduced::Other(_) => todo!(),
            //     }
            // }
            // (FluffyTerm::Unresolved(_), NestedFluffyTerm::Ethereal(_)) => todo!(),
            // (FluffyTerm::Unresolved(_), FluffyTerm::Unresolved(_)) => todo!(),
            // _ => todo!(),
        }
    }

    pub fn new_leashed(
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ty: FluffyTerm,
    ) -> EtherealTermResult<Self> {
        let function: FluffyTerm = engine.term_menu().leash_ty_ontology().into();
        Self::new_application(engine, expr_idx, function, ty)
    }

    pub(crate) fn new_ty_ontology(
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FluffyTerm; 2]>,
    ) -> Self {
        if arguments.len() == 0 {
            TermEntityPath::TypeOntology(path).into()
        } else {
            let mut merger = FluffyTermDataKindMerger::new(fluffy_terms);
            merger.accept(arguments.iter().copied());
            match merger.data_kind() {
                FluffyTermDataKind::Ethereal => todo!(),
                FluffyTermDataKind::Solid => todo!(),
                FluffyTermDataKind::Hollow => fluffy_terms
                    .hollow_terms_mut()
                    .alloc_new(HollowTermData::TypeOntology {
                        path,
                        refined_path,
                        arguments,
                    })
                    .into(),
                FluffyTermDataKind::Err => todo!(),
            }
        }
    }
}
