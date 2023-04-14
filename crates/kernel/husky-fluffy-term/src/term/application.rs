use super::*;

impl FluffyTerm {
    pub fn new_application(
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        function: impl Into<NestedFluffyTerm>,
        argument: impl Into<NestedFluffyTerm>,
    ) -> TermResult<Self> {
        let db = engine.db();
        let function = function.into();
        let argument = argument.into();
        match (function, argument) {
            (NestedFluffyTerm::Ethereal(function), NestedFluffyTerm::Ethereal(argument)) => {
                Ok(TermApplication::new(db, function, argument)?.into())
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
                        path,
                        refined_path,
                        arguments,
                    } => {
                        let mut arguments = arguments.to_smallvec();
                        arguments.push(argument.into());
                        HollowTermData::TypeOntology {
                            path,
                            refined_path,
                            arguments,
                        }
                    }
                    FluffyTermData::PlaceTypeOntology {
                        place,
                        path,
                        refined_path,
                        arguments: argument_tys,
                    } => todo!(),
                    FluffyTermData::Curry {
                        curry_kind,
                        variance,
                        parameter_variable,
                        parameter_ty,
                        return_ty,
                    } => todo!(),
                    FluffyTermData::Hole(_, _) => todo!(),
                    FluffyTermData::Category(_) => todo!(),
                    FluffyTermData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                    } => todo!(),
                    FluffyTermData::PlaceHole {
                        place,
                        hole_kind,
                        hole,
                    } => todo!(),
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
}
