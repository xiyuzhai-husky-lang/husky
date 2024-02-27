use super::*;

impl FlyTerm {
    #[inline(always)]
    pub fn new_application(
        engine: &mut impl FlyTermEngineMut,
        function: impl Into<FlyTerm>,
        argument: impl Into<FlyTerm>,
    ) -> EthTermResult<Self> {
        let db = engine.db();
        let function = function.into();
        let argument = argument.into();
        match (
            function.base_resolved(engine),
            argument.base_resolved(engine),
        ) {
            (FlyTermBase::Eth(function), FlyTermBase::Eth(argument)) => {
                Ok(EthApplication::new(db, function, argument)?.into())
            }
            (
                FlyTermBase::Eth(_) | FlyTermBase::Sol(_),
                FlyTermBase::Eth(_) | FlyTermBase::Sol(_),
            ) => {
                todo!()
            }
            _ => {
                let data = match function.data(engine) {
                    FlyTermData::TypeOntology {
                        ty_path: path,
                        refined_ty_path: refined_path,
                        ty_arguments: arguments,
                        ..
                    } => {
                        let mut arguments = arguments.to_smallvec();
                        arguments.push(argument.into());
                        HolTermData::TypeOntology {
                            path,
                            refined_path,
                            arguments,
                        }
                    }
                    FlyTermData::Hole(_, _) => todo!(),
                    FlyTermData::Symbol { .. } => todo!(),
                    FlyTermData::Hvar { .. } => todo!(),
                    FlyTermData::TypeVariant { .. } => todo!(),
                    FlyTermData::Sort(_)
                    | FlyTermData::Literal(_)
                    | FlyTermData::Curry { .. }
                    | FlyTermData::Ritchie { .. } => unreachable!(),
                };
                Ok(HolTerm::new(engine, data).into())
            }
        }
    }

    pub fn new_leashed(engine: &mut impl FlyTermEngineMut, ty: FlyTerm) -> EthTermResult<Self> {
        let function: FlyTerm = engine.term_menu().leash_ty_ontology().into();
        Self::new_application(engine, function, ty)
    }

    pub fn new_ty_ontology(
        db: &::salsa::Db,
        fly_terms: &mut FlyTerms,
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FlyTerm; 2]>,
    ) -> Self {
        if arguments.len() == 0 {
            ItemPathTerm::TypeOntology(path).into()
        } else {
            let mut merger = FlyTermDataKindMerger::new(fly_terms);
            merger.accept(arguments.iter().copied());
            match merger.data_kind() {
                FlyTermDataKind::Ethereal => {
                    match EthTerm::new_ty_ontology(
                        db,
                        path,
                        arguments.iter().map(|argument| {
                            match argument.resolve_progress(fly_terms) {
                                TermResolveProgress::ResolvedEth(argument) => argument,
                                _ => unreachable!(),
                            }
                        }),
                    ) {
                        Ok(term) => term.into(),
                        Err(_) => todo!(),
                    }
                }
                FlyTermDataKind::Solid => fly_terms
                    .solid_terms_mut()
                    .intern_new(SolidTermData::TypeOntology {
                        path,
                        refined_path,
                        arguments,
                    })
                    .into(),
                FlyTermDataKind::Hollow => fly_terms
                    .hollow_terms_mut()
                    .alloc_new(HolTermData::TypeOntology {
                        path,
                        refined_path,
                        arguments,
                    })
                    .into(),
                FlyTermDataKind::Err => todo!(),
            }
        }
    }
}
