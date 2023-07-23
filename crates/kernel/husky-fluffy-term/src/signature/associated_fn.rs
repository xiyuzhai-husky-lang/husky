use super::*;
use husky_coword::Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct AssociatedFnFluffySignature {
    parenic_parameters: SmallVec<[FluffyTermRitchieParameter; 4]>,
    return_ty: FluffyTerm,
    ty: FluffyTerm,
}

impl AssociatedFnFluffySignature {
    pub fn parenic_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.parenic_parameters
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }
}

pub(crate) fn ty_associated_fn_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    template: TypeAssociatedFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    associated_fn_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<AssociatedFnFluffySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiation = FluffyTermInstantiation::default();
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiation.try_add_rule(src, dst.into()))?;
    let mut associated_fn_template_argument_iter = associated_fn_template_arguments.iter();
    for _ in template.generic_parameters(db).iter() {
        todo!()
    }
    JustOk(AssociatedFnFluffySignature {
        parenic_parameters: template
            .parenic_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &mut instantiation))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &mut instantiation),
        ty: template
            .ty(db)
            .instantiate(engine, expr_idx, &mut instantiation),
    })
}
