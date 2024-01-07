use super::*;

impl FluffyTerm {
    #[inline(always)]
    pub fn new_application(
        engine: &mut impl FluffyTermEngine,
        function: impl Into<FluffyTerm>,
        argument: impl Into<FluffyTerm>,
    ) -> EtherealTermResult<Self> {
        let db = engine.db();
        let function = function.into();
        let argument = argument.into();
        match (
            function.base_resolved(engine),
            argument.base_resolved(engine),
        ) {
            (FluffyTermBase::Ethereal(function), FluffyTermBase::Ethereal(argument)) => {
                Ok(EtherealTermApplication::new(db, function, argument)?.into())
            }
            (
                FluffyTermBase::Ethereal(_) | FluffyTermBase::Solid(_),
                FluffyTermBase::Ethereal(_) | FluffyTermBase::Solid(_),
            ) => {
                todo!()
            }
            _ => {
                let data = match function.data(engine) {
                    FluffyTermData::TypeOntology {
                        ty_path: path,
                        refined_ty_path: refined_path,
                        ty_arguments: arguments,
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
                    FluffyTermData::Hole(_, _) => todo!(),
                    FluffyTermData::Symbol { .. } => todo!(),
                    FluffyTermData::Rune { .. } => todo!(),
                    FluffyTermData::TypeVariant { .. } => todo!(),
                    FluffyTermData::Category(_)
                    | FluffyTermData::Literal(_)
                    | FluffyTermData::Curry { .. }
                    | FluffyTermData::Ritchie { .. } => unreachable!(),
                };
                Ok(HollowTerm::new(engine, data).into())
            }
        }
    }

    pub fn new_leashed(
        engine: &mut impl FluffyTermEngine,
        ty: FluffyTerm,
    ) -> EtherealTermResult<Self> {
        let function: FluffyTerm = engine.term_menu().leash_ty_ontology().into();
        Self::new_application(engine, function, ty)
    }

    pub fn new_ty_ontology(
        db: &::salsa::Db,
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
                FluffyTermDataKind::Ethereal => {
                    match EtherealTerm::new_ty_ontology(
                        db,
                        path,
                        arguments.iter().map(|argument| {
                            match argument.resolve_progress(fluffy_terms) {
                                TermResolveProgress::ResolvedEthereal(argument) => argument,
                                _ => unreachable!(),
                            }
                        }),
                    ) {
                        Ok(term) => term.into(),
                        Err(_) => todo!(),
                    }
                }
                FluffyTermDataKind::Solid => fluffy_terms
                    .solid_terms_mut()
                    .intern_new(SolidTermData::TypeOntology {
                        path,
                        refined_path,
                        arguments,
                    })
                    .into(),
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
