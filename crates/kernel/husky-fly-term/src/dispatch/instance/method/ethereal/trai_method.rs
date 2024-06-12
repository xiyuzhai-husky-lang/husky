use super::*;
use assoc_item::trai_for_ty_item::method::method_ritchie::TraitForTypeMethodRitchieFlySignature;
use husky_entity_path::path::major_item::{
    trai::TraitPath,
    ty::{PreludeIndirectionTypePath, PreludeTypePath},
};
use husky_eth_signature::{
    helpers::trai_for_ty::trai_path_for_ty_term_impl_block_eth_signature_builders,
    signature::{
        assoc_item::trai_for_ty_item::TraitForTypeItemEtherealSignatureBuilder,
        impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilderItd,
    },
};
use husky_eth_term::term::application::TermFunctionReduced;
use vec_like::SmallVecPairMap;

impl HasFlyTraitMethodDispatch for EthTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: AvailableTraitItemsWithGivenIdent,
        mut indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyMethodDynamicDispatch> {
        let db = engine.db();
        let application_expansion = self.application_expansion(db);
        let arguments = application_expansion.arguments(db);
        let mut esbuilders_per_trai: SmallVecPairMap<
            TraitPath,
            SmallVec<[EthTraitForTypeImplBlockSignatureBuilderItd; 2]>,
            2,
        > = Default::default();
        let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
            unreachable!()
        };
        for (trai_path, trai_item_path, range) in trai_item_records {
            // todo: check scope
            let builders = trai_path_for_ty_term_impl_block_eth_signature_builders(
                db,
                trai_path,
                self,
                engine.context_itd(),
            )?;
            if !builders.is_empty() {
                unsafe { esbuilders_per_trai.insert_new_unchecked((trai_path, builders)) }
            }
        }
        match esbuilders_per_trai.len() {
            0 => match ty_path.refine(db) {
                Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
                    match prelude_indirection_ty_path {
                        PreludeIndirectionTypePath::Ref => todo!(),
                        PreludeIndirectionTypePath::RefMut => todo!(),
                        PreludeIndirectionTypePath::Leash => {
                            indirections.add(FlyIndirection::Leash);
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
                let (trai_path, ref matches) = esbuilders_per_trai.data()[0];
                match matches.len() {
                    0 => unreachable!(),
                    1 => {
                        let impl_block_signature_builder = matches[0];
                        // todo: check scope
                        let TraitForTypeItemEtherealSignatureBuilder::Method(
                            method_signature_builder,
                        ) = impl_block_signature_builder
                            .assoc_item_eth_template(db, ident_token.ident())?
                        else {
                            todo!()
                        };
                        match method_signature_builder.try_finish(db) {
                            Some(eth_sig) => JustOk(FlyInstanceDispatch {
                                signature: TraitForTypeMethodRitchieFlySignature::from_eth(
                                    indirections.final_place(),
                                    eth_sig,
                                )
                                .into(),
                                indirections,
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
