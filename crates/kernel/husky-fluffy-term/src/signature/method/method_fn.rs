use super::*;
use husky_regional_token::IdentRegionalToken;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct MethodFnFluffySignature {
    pub path: AssociatedItemPath,
    pub self_value_parameter: FluffyRitchieRegularParameter,
    pub parenate_parameters: SmallVec<[FluffyRitchieParameter; 4]>,
    pub return_ty: FluffyTerm,
    pub instantiation: FluffyInstantiation,
}

impl MemberSignature for MethodFnFluffySignature {
    fn expr_ty(&self, self_value_final_place: FluffyPlace) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}

impl MethodFnFluffySignature {
    pub(crate) fn from_ethereal(
        self_place: FluffyPlace,
        eth_sig: &TraitForTypeMethodFnEtherealSignature,
    ) -> Self {
        Self {
            path: eth_sig.path().into(),
            self_value_parameter: eth_sig.self_value_parameter.into(),
            parenate_parameters: eth_sig
                .parenate_parameters()
                .iter()
                .map(|&param| param.into())
                .collect(),
            return_ty: eth_sig.return_ty().into(),
            instantiation: FluffyInstantiation::from_ethereal(
                FluffyInstantiationEnvironment::MethodFn { self_place },
                eth_sig.instantiation(),
            ),
        }
    }

    pub fn instantiation(&self) -> &FluffyInstantiation {
        &self.instantiation
    }
}

// impl From<&TraitForTypeMethodFnEtherealSignature> for MethodFnFluffySignature {
//     fn from(sig: &TraitForTypeMethodFnEtherealSignature) -> Self {
//         MethodFnFluffySignature {
//             path: sig.path().into(),
//             parenate_parameters: sig
//                 .parenate_parameters()
//                 .iter()
//                 .map(|&param| param.into())
//                 .collect(),
//             return_ty: sig.return_ty().into(),
//             instantiation: todo!(),
//         }
//     }
// }

impl MethodFnFluffySignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FluffyRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }

    pub fn path(&self) -> AssociatedItemPath {
        self.path
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
    self_place: FluffyPlace,
) -> FluffyTermMaybeResult<MethodFnFluffySignature> {
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
                    self_place,
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
    self_place: FluffyPlace,
) -> FluffyTermMaybeResult<MethodFnFluffySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let mut instantiation_builder = FluffyInstantiationBuilder::new_associated(
        FluffyInstantiationEnvironment::MethodFn { self_place },
        template
            .path(db)
            .impl_block(db)
            .ethereal_signature_template(db)?
            .template_parameters(db),
        template.template_parameters(db),
        db,
    );
    // FluffyInstantiation::new(FluffyInstantiationEnvironment::MethodFn { self_place });
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiation_builder.try_add_rule(src, dst.into()))?;
    let mut method_template_argument_iter = method_template_arguments.iter();
    for template_parameter in template.template_parameters(db).iter() {
        match template_parameter.symbol().index(db).inner() {
            EtherealTermSymbolIndexInner::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::Type {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::Prop { disambiguator } => todo!(),
            EtherealTermSymbolIndexInner::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::ConstOther {
                attrs,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemPathLeading {
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemOther { disambiguator } => todo!(),
            EtherealTermSymbolIndexInner::SelfType => unreachable!(),
            EtherealTermSymbolIndexInner::SelfValue => todo!(),
            EtherealTermSymbolIndexInner::SelfLifetime
            | EtherealTermSymbolIndexInner::SelfPlace => continue,
        }
    }
    if let Some(_) = method_template_argument_iter.next() {
        todo!()
    }
    JustOk(MethodFnFluffySignature {
        path: template.path(db).into(),
        self_value_parameter: template.self_value_parameter(db).instantiate(
            engine,
            expr_idx,
            &mut instantiation_builder,
        ),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &mut instantiation_builder))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &mut instantiation_builder),
        instantiation: instantiation_builder.finish(db),
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
