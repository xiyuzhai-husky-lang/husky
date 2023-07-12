use super::*;
use husky_coword::Ident;

#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum MethodFluffySignature {
    MethodFn(MethodFnFluffySignature),
    MethodFunction(MethodFunctionFluffySignature),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodFnFluffySignature {
    // todo: self_parameter_contracted_ty
    params: SmallVec<[FluffyTermRitchieParameter; 4]>,
    return_ty: FluffyTerm,
}

impl MethodFnFluffySignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.params
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodFunctionFluffySignature {}

pub(crate) fn ty_method_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    ty_path: TypePath,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyTermMaybeResult<MethodFluffySignature> {
    match ty_path.ty_item_ethereal_signature_templates(engine.db(), ident)? {
        TypeItemEtherealSignatureTemplates::MethodFn(templates) => {
            for template in templates.iter().copied() {
                if let JustOk(signature) = ty_method_fn_fluffy_signature(
                    engine,
                    template,
                    ty_template_arguments,
                    method_template_arguments,
                ) {
                    return JustOk(signature.into());
                }
            }
            Nothing
        }
        TypeItemEtherealSignatureTemplates::MethodFunction(templates) => {
            for template in templates {
                if let JustOk(signature) = ty_method_function_fluffy_signature(
                    engine,
                    template,
                    ty_template_arguments,
                    method_template_arguments,
                ) {
                    return JustOk(signature.into());
                }
            }
            Nothing
        }
        TypeItemEtherealSignatureTemplates::AssociatedFn(_) => todo!(),
        TypeItemEtherealSignatureTemplates::MemoizedField(_) => todo!(),
    }
}

fn ty_method_fn_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    template: TypeMethodFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<MethodFnFluffySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiator = FluffyInstantiator::default();
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiator.try_add_rule(src, dst.into()))?;
    let mut method_template_argument_iter = method_template_arguments.iter();
    for _ in template.implicit_parameters(db).iter() {
        todo!()
    }
    JustOk(MethodFnFluffySignature {
        params: template
            .explicit_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, &mut instantiator))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, &mut instantiator),
    })
}

fn ty_method_function_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    template: &TypeMethodFunctionEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<MethodFnFluffySignature> {
    todo!()
}
