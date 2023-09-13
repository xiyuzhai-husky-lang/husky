use super::*;
use husky_coword::Ident;
use husky_regional_token::IdentRegionalToken;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
#[enum_class::from_variants]
pub enum MethodFluffySignature {
    MethodFn(MethodFnFluffySignature),
    MethodFunction(MethodFunctionFluffySignature),
    MethodGn,
}

impl From<&TraitForTypeMethodFnEtherealSignature> for MethodFluffySignature {
    fn from(sig: &TraitForTypeMethodFnEtherealSignature) -> Self {
        MethodFluffySignature::MethodFn(MethodFnFluffySignature {
            parenate_parameters: sig
                .parenate_parameters()
                .iter()
                .map(|&param| param.into())
                .collect(),
            return_ty: sig.return_ty().into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct MethodFnFluffySignature {
    // todo: self_parameter_contracted_ty
    parenate_parameters: SmallVec<[FluffyTermRitchieParameter; 4]>,
    return_ty: FluffyTerm,
}

impl MethodFnFluffySignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodFunctionFluffySignature {}

pub(crate) fn ty_method_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
    ident_token: IdentRegionalToken,
) -> FluffyTermMaybeResult<MethodFluffySignature> {
    let ident = ident_token.ident();
    match ty_path.ty_item_ethereal_signature_templates(engine.db(), ident)? {
        TypeItemEtherealSignatureTemplates::MethodFn(templates) => {
            for template in templates.iter().copied() {
                if let JustOk(signature) = ty_method_fn_fluffy_signature(
                    engine,
                    expr_idx,
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
    expr_idx: SynExprIdx,
    template: TypeMethodFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<MethodFnFluffySignature> {
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
    let mut method_template_argument_iter = method_template_arguments.iter();
    for _ in template.template_parameters(db).iter() {
        todo!()
    }
    JustOk(MethodFnFluffySignature {
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &mut instantiation))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &mut instantiation),
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
