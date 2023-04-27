use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum FluffyMethodSignature {
    MethodFn(FluffyMethodFnSignature),
    MethodFunction(FluffyMethodFunctionSignature),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FluffyMethodFnSignature {
    // todo: self_parameter_contracted_ty
    nonself_parameter_contracted_tys: SmallVec<[FluffyTermRitchieParameterContractedType; 4]>,
    return_ty: FluffyTerm,
}

impl FluffyMethodFnSignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameterContractedType] {
        &self.nonself_parameter_contracted_tys
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FluffyMethodFunctionSignature {}

pub(crate) fn ty_method_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    ty_path: TypePath,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodSignature> {
    let templates = ty_path.ty_method_ethereal_signature_templates(engine.db(), ident)?;
    match templates {
        TypeMethodEtherealSignatureTemplates::MethodFn(templates) => {
            for template in templates {
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
        TypeMethodEtherealSignatureTemplates::MethodFunction(templates) => {
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
    }
}

fn ty_method_fn_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    template: &TypeMethodFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
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
    let mut method_template_argument_iter = method_template_arguments.iter();
    for _ in template.implicit_parameters(db).iter() {
        todo!()
    }
    JustOk(FluffyMethodFnSignature {
        nonself_parameter_contracted_tys: template
            .nonself_regular_parameters(db)
            .iter()
            .map(|v| instantiator.instantiate_ritchie_parameter(engine, v))
            .collect(),
        return_ty: instantiator.instantiate_term(engine, template.return_ty(db)),
    })
}

fn ty_method_function_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    template: &TypeMethodFunctionEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
    todo!()
}
