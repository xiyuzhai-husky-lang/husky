use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct AssociatedFnFluffySignature {
    path: AssociatedItemPath,
    parenate_parameters: SmallVec<[FluffyRitchieParameter; 4]>,
    return_ty: FluffyTerm,
    ty: FluffyTerm,
    instantiation: FluffyInstantiation,
}

impl AssociatedFnFluffySignature {
    pub fn parenate_parameter_contracted_tys(&self) -> &[FluffyRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    pub fn path(&self) -> AssociatedItemPath {
        self.path
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
    let mut instantiation_builder = FluffyInstantiationBuilder::new_associated(
        FluffyInstantiationEnvironment::AssociatedFn,
        template
            .path(db)
            .impl_block(db)
            .ethereal_signature_template(db)?
            .template_parameters(db),
        template.template_parameters(db),
        db,
    );
    std::iter::zip(
        self_ty_application_expansion.arguments(db),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(&src, dst)| instantiation_builder.try_add_rule(src, dst.into()))?;
    std::iter::zip(
        template.template_parameters(db),
        associated_fn_template_arguments,
    )
    .try_for_each(|(src, &dst)| {
        instantiation_builder.try_add_rule(src.symbol().into(), dst.into())
    })?;
    let instantiation = instantiation_builder.finish(db);
    JustOk(AssociatedFnFluffySignature {
        path: template.path(db).into(),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &instantiation))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &instantiation),
        ty: template
            .ty(db)
            .instantiate(engine, expr_idx, &instantiation),
        instantiation,
    })
}
