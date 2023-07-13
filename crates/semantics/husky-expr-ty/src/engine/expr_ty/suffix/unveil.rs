use husky_ethereal_signature::{
    EtherealSignatureError, EtherealSignatureMaybeResult, EtherealSignatureResult,
    HasTypeSideTraitForTypeImplBlockSignatureTemplates,
    TraitForTypeImplBlockEtherealSignatureTemplate,
    TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated,
    TraitForTypeImplBlockSignature,
};
use maybe_result::*;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_unveil_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        match self.unveiler {
            Unveiler::Unique { .. } => {
                // self.infer_new_expr_ty_discarded(
                //     opd,
                //     ExpectCoersion::new(Contract::Move, ty.into()),
                // );
                todo!()
            }
            Unveiler::Nothing => Err(OriginalExprTypeError::CannotUnveil)?,
            Unveiler::ErrUnableToInferReturnTypeForUnveiling => {
                Err(DerivedExprTypeError::UnableToInferReturnTypeForUnveiling)?
            }
            Unveiler::ErrEtherealSignature(e) => Err(e.into()),
        }
    }
}

pub(crate) enum Unveiler {
    Unique {
        opd_ty: EtherealTerm,
        unveil_output_ty: EtherealTerm,
        unveil_output_ty_final_destination: FinalDestination,
    },
    Nothing,
    ErrUnableToInferReturnTypeForUnveiling,
    ErrEtherealSignature(EtherealSignatureError),
}

impl Unveiler {
    pub(crate) fn new(db: &dyn ExprTypeDb, return_ty: Option<EtherealTerm>) -> Self {
        let Some(return_ty) = return_ty else {
            return Unveiler::ErrUnableToInferReturnTypeForUnveiling
        };
        match Self::new_aux(db, return_ty) {
            MaybeResult::JustOk(unveiler) => unveiler,
            MaybeResult::JustErr(e) => Unveiler::ErrEtherealSignature(e),
            MaybeResult::Nothing => Unveiler::Nothing,
        }
    }

    fn new_aux(db: &dyn ExprTypeDb, return_ty: EtherealTerm) -> EtherealSignatureMaybeResult<Self> {
        let templates = unveil_impl_block_signature_templates(db, return_ty)?;
        match templates.len() {
            0 => todo!(),
            1 => {
                todo!()
                // let template = templates[0];
                // match template.implicit_parameters(db).len() {
                //     0 => {
                //         let trai_arguments =
                //             template.trai(db).application_expansion(db).arguments(db);
                //         debug_assert_eq!(trai_arguments.len(), 1);
                //         let unveil_output_ty = template.associated_output_term(db)?;
                //         let unveil_output_ty_final_destination = todo!();
                //         JustOk(Unveiler::Unique {
                //             opd_ty: trai_arguments[0],
                //             unveil_output_ty,
                //             unveil_output_ty_final_destination,
                //         })
                //     }
                //     _ => todo!(),
                // }
            }
            _ => todo!(),
        }
    }
}

fn unveil_impl_block_signature_templates(
    db: &dyn ExprTypeDb,
    term: EtherealTerm,
) -> EtherealSignatureMaybeResult<
    &[TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated],
> {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(_) => {
            // ad hoc
            Nothing
        }
        EtherealTerm::Variable(_) => todo!(),
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Fugitive(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => {
                ty_ontology_path_unveil_impl_block_signature_templates(db, path).just_ok_as_ref2()
            }
            TermEntityPath::TypeInstance(_) => todo!(),
            TermEntityPath::TypeVariant(_) => todo!(),
        },
        EtherealTerm::Category(_) => todo!(),
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(_) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(path) => {
            ty_ontology_application_unveil_impl_block_signature_templates(db, path)
                .just_ok_as_ref2()
        }
        EtherealTerm::Subentity(_) => todo!(),
        EtherealTerm::AsTraitSubentity(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
fn ty_ontology_path_unveil_impl_block_signature_templates(
    db: &dyn ExprTypeDb,
    ty_path: TypePath,
) -> EtherealSignatureMaybeResult<
    SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated; 2]>,
> {
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        &[],
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)),
    )
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
fn ty_ontology_application_unveil_impl_block_signature_templates(
    db: &dyn ExprTypeDb,
    ty_target: EtherealTermApplication,
) -> EtherealSignatureMaybeResult<
    SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated; 2]>,
> {
    let application_expansion = ty_target.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
        todo!()
    };
    unveil_impl_block_signature_templates_aux(
        db,
        ty_path,
        application_expansion.arguments(db),
        ty_target.into(),
    )
}

fn unveil_impl_block_signature_templates_aux(
    db: &dyn ExprTypeDb,
    ty_path: TypePath,
    arguments: &[EtherealTerm],
    ty_target: EtherealTerm,
) -> EtherealSignatureMaybeResult<
    SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplatePartialInstantiated; 2]>,
> {
    let entity_path_menu = db.entity_path_menu(ty_path.toolchain(db));
    let templates = ty_path.ty_side_trai_for_ty_impl_block_signature_templates(
        db,
        entity_path_menu.core_ops_unveil_trai_path(),
    )?;
    JustOk(
        templates
            .iter()
            .map(|template| template.instantiate_ty(db, arguments, ty_target))
            .collect::<EtherealSignatureResult<_>>()?,
    )
}
