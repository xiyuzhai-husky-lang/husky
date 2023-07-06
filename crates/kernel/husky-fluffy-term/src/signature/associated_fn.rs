use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
pub struct AssociatedFnFluffySignature {
    explicit_parameters: SmallVec<[FluffyTermRitchieParameter; 4]>,
    return_ty: FluffyTerm,
    ty: FluffyTerm,
}

impl AssociatedFnFluffySignature {
    pub fn explicit_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.explicit_parameters
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }
}

pub(crate) fn ty_associated_fn_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    template: TypeAssociatedFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    associated_fn_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<AssociatedFnFluffySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiator = Instantiator::default();
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiator.try_add_rule(src, dst.into()))?;
    let mut associated_fn_template_argument_iter = associated_fn_template_arguments.iter();
    for _ in template.implicit_parameters(db).iter() {
        todo!()
    }
    JustOk(AssociatedFnFluffySignature {
        explicit_parameters: template
            .explicit_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, &mut instantiator))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, &mut instantiator),
        ty: template.ty(db).instantiate(engine, &mut instantiator),
    })
}
