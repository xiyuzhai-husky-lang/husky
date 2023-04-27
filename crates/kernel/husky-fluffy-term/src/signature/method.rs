use super::*;
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub enum FluffyMethodSignature {
    MethodFn(FluffyMethodFnSignature),
    MethodFunction(FluffyMethodFunctionSignature),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FluffyMethodFnSignature {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FluffyMethodFunctionSignature {}

pub(crate) fn ty_method_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodSignature> {
    let templates = ty_path.ty_method_ethereal_signature_templates(db, ident)?;
    match templates {
        TypeMethodEtherealSignatureTemplates::MethodFn(templates) => {
            for template in templates {
                if let JustOk(signature) = ty_method_fn_fluffy_signature(
                    db,
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
                    db,
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
    db: &dyn FluffyTermDb,
    template: &TypeMethodFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiator = Instantiator::default();
    // initialize pattern matcher
    for (src, dst) in std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    ) {
        let dst: FluffyTerm = dst.into();
        todo!()
    }
    for _ in method_template_arguments {
        todo!()
    }
    JustOk(FluffyMethodFnSignature {})
}

fn ty_method_function_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    db: &dyn FluffyTermDb,
    template: &TypeMethodFunctionEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
    todo!()
}
