use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct AssociatedFnFlySignature {
    path: AssociatedItemPath,
    parenate_parameters: SmallVec<[FlyRitchieParameter; 4]>,
    return_ty: FlyTerm,
    ty: FlyTerm,
    instantiation: FlyInstantiation,
    self_ty: FlyTerm,
}

impl AssociatedFnFlySignature {
    pub fn parenate_parameter_contracted_tys(&self) -> &[FlyRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn ty(&self) -> FlyTerm {
        self.ty
    }

    pub fn path(&self) -> AssociatedItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &FlyInstantiation {
        &self.instantiation
    }

    pub fn self_ty(&self) -> FlyTerm {
        self.self_ty
    }
}

pub(crate) fn ty_associated_fn_fluffy_signature<Term: Copy + Into<FlyTerm>>(
    engine: &mut impl FlyTermEngine,
    expr_idx: SynExprIdx,
    template: TypeAssociatedFnEthTemplate,
    ty_template_arguments: &[Term],
    associated_fn_template_arguments: &[FlyTerm],
) -> FlyTermMaybeResult<AssociatedFnFlySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiation_builder = FlyTermInstantiationBuilder::new_associated(
        template.path(db),
        FlyInstantiationEnvironment::AssociatedFn,
        template
            .path(db)
            .impl_block(db)
            .eth_template(db)?
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
    JustOk(AssociatedFnFlySignature {
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
        self_ty: template
            .self_ty(db)
            .instantiate(engine, expr_idx, &instantiation),
        instantiation,
    })
}
