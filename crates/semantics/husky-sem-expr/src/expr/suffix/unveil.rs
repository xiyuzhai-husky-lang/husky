use super::*;
use context::EthTermContextItd;
use husky_coword::coword_menu;
use husky_entity_path::{
    menu::item_path_menu,
    path::major_item::ty::{PreludeIndirectionTypePath, TypePath},
};
use husky_entity_tree::node::HasAssocItemPaths;
use husky_eth_signature::{
    error::{EthSignatureError, EthSignatureMaybeResult, EthSignatureResult},
    helpers::trai_for_ty::*,
    signature::{
        impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilderItd,
        package::PackageEthSignatureData,
    },
};
use husky_eth_term::term::{
    application::{EthApplication, TermFunctionReduced},
    EthTerm,
};
use maybe_result::*;
use vec_like::VecMapGetEntry;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_unveil_expr_ty(
        &mut self,
        opd_syn_expr_idx: SynExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let db = self.db();
        self.unveiler
            .initialize_if_not(self.return_ty(), self.context_itd(), db);
        match self.unveiler {
            Unveiler::UniqueFullyInstantiated {
                opd_ty,
                unveil_output_ty,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                ..
            } => {
                let unveil_output_ty_signature = unveil_output_ty_signature.clone();
                let opd_sem_expr_idx = self.build_expr(
                    opd_syn_expr_idx,
                    ExpectCoercion::new(Contract::Move, opd_ty.into()),
                );
                (
                    Ok(SemExprData::Unveil {
                        opd: opd_sem_expr_idx,
                        opr_regional_token_idx,
                        unveil_output_ty_signature,
                        unveil_assoc_fn_path,
                        return_ty: self.return_ty().unwrap(),
                    }),
                    Ok(unveil_output_ty.into()),
                )
            }
            Unveiler::UniquePartiallyInstanted { template } => {
                let (opd_sem_expr_idx, opd_ty) =
                    self.build_expr_with_ty(opd_syn_expr_idx, ExpectAnyOriginal);
                let Some(opd_ty) = opd_ty else {
                    // p!(self.syn_expr_region_data.path().debug(db));
                    // p!(self.syn_expr_region_data[opd_syn_expr_idx].debug(db));
                    todo!()
                };
                let reduced_opd_ty: FlyTerm = match opd_ty.base_ty_data(self) {
                    FlyBaseTypeData::TypeOntology {
                        ty_path,
                        refined_ty_path: Left(PreludeTypePath::Indirection(indirection_path)),
                        ty_arguments,
                        ty_ethereal_term,
                    } => {
                        match indirection_path {
                            PreludeIndirectionTypePath::Ref => todo!(),
                            PreludeIndirectionTypePath::RefMut => todo!(),
                            PreludeIndirectionTypePath::Leash => {
                                // ad hoc
                                // needs to check more
                                ty_arguments[0]
                            }
                            PreludeIndirectionTypePath::At => todo!(),
                        }
                    }
                    _ => opd_ty,
                };
                match reduced_opd_ty.base_resolved(self) {
                    FlyTermBase::Eth(opd_ty) => match template.instantiate_trai(&[opd_ty], db) {
                        JustOk(template) => {
                            let assoc_output_template = match template.assoc_output_template(db) {
                                Ok(assoc_output_template) => assoc_output_template,
                                Err(e) => {
                                    return (
                                        Err(DerivedSemExprDataError::UnveilOutputTemplate {
                                            opd_sem_expr_idx,
                                            e,
                                        }
                                        .into()),
                                        Err(e.into()),
                                    )
                                }
                            };
                            let Some(unveil_output_ty_signature) =
                                assoc_output_template.try_into_signature(db)
                            else {
                                todo!()
                            };
                            let ty_term = unveil_output_ty_signature.ty_term().into();
                            (
                                Ok(SemExprData::Unveil {
                                    opd: opd_sem_expr_idx,
                                    opr_regional_token_idx,
                                    unveil_assoc_fn_path: unveil_assoc_fn_path(
                                        &unveil_output_ty_signature,
                                        db,
                                    ),
                                    unveil_output_ty_signature,
                                    return_ty: self.return_ty().unwrap(),
                                }),
                                Ok(ty_term),
                            )
                        }
                        JustErr(_) => todo!(),
                        Nothing => todo!(),
                    },
                    FlyTermBase::Sol(_) => todo!(),
                    FlyTermBase::Hol(_) => todo!(),
                    FlyTermBase::Place => todo!(),
                }
            }
            Unveiler::Nothing => (
                Err(todo!()),
                Err(OriginalSemExprTypeError::CannotUnveil.into()),
            ),
            Unveiler::ErrUnableToInferReturnTypeForUnveiling => (
                Err(todo!()),
                Err(DerivedSemExprTypeError::UnableToInferReturnTypeForUnveiling.into()),
            ),
            Unveiler::ErrEthSignature(e) => (Err(todo!()), Err(e.into())),
            Unveiler::Uninitialized => unreachable!(),
        }
    }

    pub(super) fn calc_unveil_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FlyTerm,
    ) -> (SemExprDataResult<SemaSuffixOpr>, SemExprTypeResult<FlyTerm>) {
        todo!()
    }
}

#[salsa::derive_debug_with_db]
pub(crate) enum Unveiler {
    Uninitialized,
    UniqueFullyInstantiated {
        opd_ty: EthTerm,
        unveil_output_ty: EthTerm,
        unveil_output_ty_final_destination: FinalDestination,
        unveil_output_ty_signature: TraitForTypeAssocTypeEthSignature,
        unveil_assoc_fn_path: TraitForTypeItemPath,
    },
    UniquePartiallyInstanted {
        template: EthTraitForTypeImplBlockSignatureBuilderItd,
    },
    Nothing,
    ErrUnableToInferReturnTypeForUnveiling,
    ErrEthSignature(EthSignatureError),
}

impl Unveiler {
    pub(crate) fn initialize_if_not<'db>(
        &mut self,
        return_ty: Option<EthTerm>,
        context_itd: EthTermContextItd,
        db: &'db ::salsa::Db,
    ) {
        match self {
            Unveiler::Uninitialized => (),
            _ => return,
        }
        let Some(return_ty) = return_ty else {
            *self = Unveiler::ErrUnableToInferReturnTypeForUnveiling;
            return;
        };
        *self = match Self::new_aux(db, return_ty, context_itd) {
            MaybeResult::JustOk(unveiler) => unveiler,
            MaybeResult::JustErr(e) => Unveiler::ErrEthSignature(e),
            MaybeResult::Nothing => Unveiler::Nothing,
        }
    }

    fn new_aux<'db>(
        db: &'db ::salsa::Db,
        return_ty: EthTerm,
        context_itd: EthTermContextItd,
    ) -> EthSignatureMaybeResult<Self> {
        let templates = unveil_impl_block_signature_templates(return_ty, context_itd, db)?;
        match templates.len() {
            0 => todo!(),
            1 => {
                let template = templates[0];
                if let Some(impl_block_signature) = template.try_into_signature(db) {
                    let unveil_output_ty_signature = template
                        .assoc_output_template(db)?
                        .try_into_signature(db)
                        .expect("no generic parameters for Unveil::Output");
                    let unveil_output_ty = unveil_output_ty_signature.ty_term();
                    JustOk(Unveiler::UniqueFullyInstantiated {
                        opd_ty: impl_block_signature
                            .trai()
                            .application_expansion(db)
                            .arguments(db)[0],
                        unveil_output_ty,
                        unveil_output_ty_final_destination: unveil_output_ty.final_destination(db),
                        unveil_assoc_fn_path: unveil_assoc_fn_path(&unveil_output_ty_signature, db),
                        unveil_output_ty_signature,
                    })
                } else {
                    JustOk(Unveiler::UniquePartiallyInstanted { template })
                }
            }
            _ => todo!(),
        }
    }
}

fn unveil_assoc_fn_path(
    unveil_output_ty_signature: &TraitForTypeAssocTypeEthSignature,
    db: &::salsa::Db,
) -> TraitForTypeItemPath {
    let snake_case_unveil_ident = coword_menu(db).snake_case_unveil_ident();
    unveil_output_ty_signature
        .path()
        .impl_block(db)
        .assoc_item_paths(db)
        .get_entry(snake_case_unveil_ident)
        .expect("unveil associated fn should exist!")
        .1
}

fn unveil_impl_block_signature_templates<'db>(
    term: EthTerm,
    context_itd: EthTermContextItd,
    db: &'db ::salsa::Db,
) -> EthSignatureMaybeResult<SmallVec<[EthTraitForTypeImplBlockSignatureBuilderItd; 2]>> {
    match term {
        EthTerm::SymbolicVariable(_) => Nothing, // ad hoc
        EthTerm::LambdaVariable(_) => Nothing,   // ad hoc
        EthTerm::ItemPath(ItemPathTerm::TypeOntology(path)) => {
            ty_ontology_path_unveil_impl_block_signature_templates(path, context_itd, db)
        }
        EthTerm::Application(path) => {
            ty_ontology_application_unveil_impl_block_signature_templates(db, path, context_itd)
        }
        EthTerm::TypeAsTraitItem(_) => todo!(),
        _ => Nothing,
    }
}

fn ty_ontology_path_unveil_impl_block_signature_templates<'db>(
    ty_path: TypePath,
    context_itd: EthTermContextItd,
    db: &'db ::salsa::Db,
) -> EthSignatureMaybeResult<SmallVec<[EthTraitForTypeImplBlockSignatureBuilderItd; 2]>> {
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        &[],
        EthTerm::ItemPath(ItemPathTerm::TypeOntology(ty_path)),
        context_itd,
    )
}

fn ty_ontology_application_unveil_impl_block_signature_templates<'db>(
    db: &'db ::salsa::Db,
    ty_target: EthApplication,
    context_itd: EthTermContextItd,
) -> EthSignatureMaybeResult<SmallVec<[EthTraitForTypeImplBlockSignatureBuilderItd; 2]>> {
    let application_expansion = ty_target.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
        todo!()
    };
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        application_expansion.arguments(db),
        ty_target.into(),
        context_itd,
    )
}

fn unveil_impl_block_signature_templates_aux<'db>(
    db: &'db ::salsa::Db,
    ty_path: TypePath,
    arguments: &[EthTerm],
    ty_target: EthTerm,
    context_itd: EthTermContextItd,
) -> EthSignatureMaybeResult<SmallVec<[EthTraitForTypeImplBlockSignatureBuilderItd; 2]>> {
    let item_path_menu = item_path_menu(db, ty_path.toolchain(db));
    let templates = ty_side_trai_for_ty_impl_block_signature_templates(
        db,
        item_path_menu.unveil_trai_path(),
        ty_path,
    )?;
    JustOk(
        templates
            .iter()
            .filter_map(|template| {
                template
                    .instantiate_ty(arguments, ty_target, context_itd, db)
                    .into_option_result()
            })
            .collect::<EthSignatureResult<_>>()?,
    )
}
