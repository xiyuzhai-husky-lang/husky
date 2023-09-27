use husky_ethereal_signature::helpers::trai_for_ty::trai_for_ty_impl_block_ethereal_signature_templates;
use salsa::DisplayWithDb;
use vec_like::SmallVecPairMap;

use super::*;

impl HasFluffyTraitMethodDispatch for EtherealTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        mut indirections: FluffyTermDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyTermMethodDynamicDispatch> {
        let db = engine.db();
        let mut matches: SmallVec<[(); 2]> = Default::default();
        let application_expansion = self.application_expansion(db);
        let arguments = application_expansion.arguments(db);
        let mut trai_path_selected: Option<TraitPath> = None;
        let mut matches_map: SmallVecPairMap<
            TraitPath,
            SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated; 2]>,
            2,
        > = Default::default();
        let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
            unreachable!()
        };
        for record in trai_item_records.records() {
            // todo: check scope
            let trai_path = record.trai_path();
            let mut matches: SmallVec<
                [TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated; 2],
            > = Default::default();
            for template in
                trai_for_ty_impl_block_ethereal_signature_templates(db, trai_path, ty_path)?.iter()
            {
                match template.instantiate_ty(db, arguments, self) {
                    JustOk(template_partially_instantiated) => {
                        matches.push(template_partially_instantiated)
                    }
                    JustErr(_) => todo!(),
                    Nothing => todo!(),
                }
            }
            if !matches.is_empty() {
                unsafe { matches_map.insert_new_unchecked((trai_path, matches)) }
            }
        }
        match matches_map.len() {
            0 => match ty_path.refine(db) {
                Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
                    match prelude_indirection_ty_path {
                        PreludeIndirectionTypePath::Ref => todo!(),
                        PreludeIndirectionTypePath::RefMut => todo!(),
                        PreludeIndirectionTypePath::Leash => {
                            indirections.add(FluffyTermDynamicDispatchIndirection::Leash);
                            debug_assert_eq!(arguments.len(), 1);
                            let the_argument = arguments[0];
                            the_argument.trai_method_dispatch_aux(
                                engine,
                                expr_idx,
                                ident_token,
                                trai_item_records,
                                indirections,
                            )
                        }
                        PreludeIndirectionTypePath::At => todo!(),
                    }
                }
                Left(_) => Nothing,
                Right(_) => {
                    // todo: consider custom Deref Carrier etc
                    Nothing
                }
            },
            1 => {
                let (trai_path, ref matches) = matches_map.data()[0];
                match matches.len() {
                    0 => unreachable!(),
                    1 => {
                        let impl_block_template_partially_instantiated = matches[0];
                        // todo: check scope
                        let TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated::Method(
                            method_template_partially_instantiated,
                        ) = impl_block_template_partially_instantiated
                            .associated_item_ethereal_signature_template(db, ident_token.ident())?
                        else {
                            todo!()
                        };
                        match method_template_partially_instantiated.try_into_signature(db) {
                            Some(signature) => JustOk(FluffyDynamicDispatch {
                                indirections,
                                signature: signature.into(),
                            }),
                            None => todo!(),
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
