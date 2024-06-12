use super::*;
use husky_entity_path::path::{assoc_item::AssocItemPath, major_item::ty::TypePath};
use husky_eth_signature::{
    context::EthSignatureBuilderContextItd,
    signature::{
        assoc_item::{
            trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieEtherealSignature,
            ty_item::{
                method_curry::TypeMethodCurryEthTemplate,
                method_ritchie::TypeMethodRitchieEthTemplate, HasTypeItemTemplates,
                TypeItemEthTemplates,
            },
        },
        HasEthTemplate,
    },
};
use husky_eth_term::term::symbolic_variable::EthTermSymbolIndexImpl;
use husky_regional_token::IdentRegionalToken;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodRitchieFlySignature {
    pub path: AssocItemPath,
    pub self_value_parameter: FlyRitchieSimpleParameter,
    pub parenate_parameters: SmallVec<[FlyRitchieParameter; 4]>,
    pub return_ty: FlyTerm,
    pub instantiation: FlyInstantiation,
}

impl MemberSignature for MethodRitchieFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}

impl MethodRitchieFlySignature {
    pub(crate) fn from_eth(
        self_place: FlyQuary,
        eth_sig: &TraitForTypeMethodRitchieEtherealSignature,
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
            instantiation: FlyInstantiation::from_eth(
                FlyInstantiationEnvironment::MethodFn { self_place },
                eth_sig.instantiation(),
            ),
        }
    }

    pub fn instantiation(&self) -> &FlyInstantiation {
        &self.instantiation
    }
}

impl MethodRitchieFlySignature {
    pub fn nonself_parameter_contracted_tys(&self) -> &[FlyRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> FlyTerm {
        self.return_ty
    }

    pub fn path(&self) -> AssocItemPath {
        self.path
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodFunctionFlySignature {}

pub(crate) fn ty_method_fly_signature<'db, Term: Copy + Into<FlyTerm>>(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FlyTerm],
    ident_token: IdentRegionalToken,
    self_place: FlyQuary,
    context_itd: EthSignatureBuilderContextItd,
) -> FlyTermMaybeResult<MethodRitchieFlySignature> {
    let ident = ident_token.ident();
    match ty_path.ty_item_eth_templates(engine.db(), ident)? {
        TypeItemEthTemplates::MethodFn(templates) => {
            for template in templates.iter().copied() {
                if let JustOk(signature) = ty_method_ritchie_fly_signature(
                    engine,
                    expr_idx,
                    template,
                    ty_template_arguments,
                    method_template_arguments,
                    self_place,
                    context_itd,
                ) {
                    return JustOk(signature.into());
                }
            }
            Nothing
        }
        TypeItemEthTemplates::MethodFunction(templates) => {
            for template in templates {
                if let JustOk(signature) = ty_method_curry_fly_signature(
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
        TypeItemEthTemplates::AssocRitchie(_) => todo!(),
        TypeItemEthTemplates::MemoizedField(_) => todo!(),
    }
}

fn ty_method_ritchie_fly_signature<'db, Term: Copy + Into<FlyTerm>>(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    template: TypeMethodRitchieEthTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FlyTerm],
    self_place: FlyQuary,
    context_itd: EthSignatureBuilderContextItd,
) -> FlyTermMaybeResult<MethodRitchieFlySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    let path = template.path(db);
    let mut instantiation_builder = FlyTermInstantiationBuilder::new_associated(
        path,
        FlyInstantiationEnvironment::MethodFn { self_place },
        path.impl_block(db)
            .eth_template(db)?
            .template_parameters(db),
        template.template_parameters(db),
        context_itd,
        db,
    );
    // FlyInstantiation::new(FlyInstantiationEnvironment::MethodFn { self_place });
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiation_builder.try_add_rule(src, dst.into()))?;
    let mut method_template_argument_iter = method_template_arguments.iter();
    for template_parameter in template.template_parameters(db).iter() {
        match template_parameter.symbol().index(db).inner() {
            EthTermSymbolIndexImpl::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EthTermSymbolIndexImpl::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EthTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EthTermSymbolIndexImpl::Prop { disambiguator } => todo!(),
            EthTermSymbolIndexImpl::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => todo!(),
            EthTermSymbolIndexImpl::ConstOther {
                attrs,
                disambiguator,
            } => todo!(),
            EthTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            } => todo!(),
            EthTermSymbolIndexImpl::EphemOther { disambiguator } => todo!(),
            EthTermSymbolIndexImpl::SelfType => unreachable!(),
            EthTermSymbolIndexImpl::SelfValue => todo!(),
            EthTermSymbolIndexImpl::SelfLifetime | EthTermSymbolIndexImpl::SelfPlace => continue,
        }
    }
    if let Some(_) = method_template_argument_iter.next() {
        todo!()
    }
    let instantiation = instantiation_builder.finish(db);
    JustOk(MethodRitchieFlySignature {
        path: template.path(db).into(),
        self_value_parameter: template.self_value_parameter(db).instantiate(
            engine,
            expr_idx,
            &instantiation,
        ),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &instantiation))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &instantiation),
        instantiation,
    })
}

fn ty_method_curry_fly_signature<Term: Copy + Into<FlyTerm>>(
    engine: &mut impl FlyTermEngineMut,
    template: &TypeMethodCurryEthTemplate,
    ty_template_arguments: &[Term],
    method_template_arguments: &[FlyTerm],
) -> FlyTermMaybeResult<MethodRitchieFlySignature> {
    todo!()
}
