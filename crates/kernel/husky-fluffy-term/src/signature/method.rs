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

pub(crate) fn method_fluffy_signature(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ty_template_arguments: impl Iterator<Item = FluffyTerm> + Clone,
    method_template_arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodSignature> {
    let templates = ty_path.ty_method_ethereal_signature_templates(db, ident)?;
    match templates {
        TypeMethodEtherealSignatureTemplates::MethodFn(templates) => {
            for template in templates {
                if let JustOk(signature) = method_fn_fluffy_signature(
                    template,
                    ty_template_arguments.clone(),
                    method_template_arguments,
                ) {
                    return JustOk(signature.into());
                }
            }
            Nothing
        }
        TypeMethodEtherealSignatureTemplates::MethodFunction(templates) => {
            for template in templates {
                if let JustOk(signature) = method_function_fluffy_signature(
                    template,
                    ty_template_arguments.clone(),
                    method_template_arguments,
                ) {
                    return JustOk(signature.into());
                }
            }
            Nothing
        }
    }
}

fn method_fn_fluffy_signature(
    template: &TypeMethodFnEtherealSignatureTemplate,
    ty_template_arguments: impl Iterator<Item = FluffyTerm>,
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
    todo!()
}

fn method_function_fluffy_signature(
    template: &TypeMethodFunctionEtherealSignatureTemplate,
    ty_template_arguments: impl Iterator<Item = FluffyTerm>,
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<FluffyMethodFnSignature> {
    todo!()
}
