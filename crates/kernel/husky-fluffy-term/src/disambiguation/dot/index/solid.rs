use super::*;

impl SolidTerm {
    pub(super) fn disambiguate_index(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        index_ty: FluffyTerm,
    ) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
        self.disambiguate_index_aux(engine, expr_idx, index_ty, Default::default())
    }

    pub(super) fn disambiguate_index_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        index_ty: FluffyTerm,
        mut indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
        let db = engine.db();
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::TypeOntologyAtPlace {
                place,
                path,
                refined_path,
                arguments,
                base_ty_term,
            } => match base_ty_term {
                Some(base_ty_term) => JustOk(
                    ethereal_owner_ty_index_disambiguation(
                        engine,
                        expr_idx,
                        *base_ty_term,
                        index_ty,
                    )?
                    .merge(indirections),
                ),
                None => todo!(),
            },
            SolidTermData::SymbolAtPlace { .. } => todo!(),
            SolidTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolidTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}

// let owner_ty_application_expansion = owner_ty.application_expansion(db);
//     let TermFunctionReduced::TypeOntology(ty_path) = owner_ty_application_expansion.function() else {
//         todo!()
//     };
//     let refined_ty_path = ty_path.refine(db);
//     let owner_ty_arguments = owner_ty_application_expansion.arguments(db);
//     if let Some(index_signature) = ethereal_owner_ty_index_signature(
//         engine,
//         expr_idx,
//         refined_ty_path,
//         owner_ty_arguments,
//         index_ty,
//     )
//     .into_result_option()?
//     {
//         return JustOk(FluffyIndexDisambiguation::new(index_signature));
//     };
//     // indirections
//     match refined_ty_path {
//         Left(prelude_ty_path) => match prelude_ty_path {
//             PreludeTypePath::Borrow(prelude_borrow_ty_path) => match prelude_borrow_ty_path {
//                 PreludeBorrowTypePath::Ref => todo!(),
//                 PreludeBorrowTypePath::RefMut => todo!(),
//                 PreludeBorrowTypePath::Leash => {
//                     indirections.push(FluffyIndirection::Leash);
//                     if owner_ty_arguments.len() != 1 {
//                         todo!()
//                     }
//                     ethereal_owner_ty_index_disambiguation_aux(
//                         engine,
//                         expr_idx,
//                         owner_ty_arguments[0],
//                         index_ty,
//                         indirections,
//                     )
//                 }
//             },
//             PreludeTypePath::Basic(_)
//             | PreludeTypePath::Num(_)
//             | PreludeTypePath::Nat
//             | PreludeTypePath::Lifetime
//             | PreludeTypePath::Module
//             | PreludeTypePath::Trait
//             | PreludeTypePath::List
//             | PreludeTypePath::Array
//             | PreludeTypePath::Array2d
//             | PreludeTypePath::Array3d
//             | PreludeTypePath::Array4d
//             | PreludeTypePath::Array5d
//             | PreludeTypePath::Slice
//             | PreludeTypePath::StringLiteral
//             | PreludeTypePath::Str => Nothing,
//         },
//         Right(_) => todo!(),
