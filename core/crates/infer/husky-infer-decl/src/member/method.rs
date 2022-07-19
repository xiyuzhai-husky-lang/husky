use crate::*;
use fold::LocalStack;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    *,
};
use husky_implement::{Implementable, Implementor};
use husky_instantiate::InstantiationContext;
use map_collect::MapCollect;
use print_utils::p;
use vec_like::VecMapEntry;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct CallFormDecl {
    pub ident: CustomIdentifier,
    pub this_liason: ParameterLiason,
    pub parameters: Vec<ParameterDecl>,
    pub output: OutputDecl,
    pub spatial_parameters: IdentDict<SpatialParameter>,
    pub is_lazy: bool,
    pub kind: MethodKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodKind {
    Type,
    Trait { trai: EntityRoutePtr },
}

impl MethodKind {
    pub fn instantiate(&self, instantiator: &InstantiationContext) -> Self {
        match self {
            MethodKind::Type => MethodKind::Type,
            MethodKind::Trait { trai } => MethodKind::Trait {
                trai: trai.instantiate(instantiator).take_entity_route(),
            },
        }
    }

    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        method_variant: MethodStaticDefnKind,
    ) -> Self {
        match method_variant {
            MethodStaticDefnKind::TypeMethod => Self::Type,
            MethodStaticDefnKind::TraitMethod => {
                // opt_this_ty,
                Self::Trait {
                    trai: symbol_context.trai(),
                }
            }
            MethodStaticDefnKind::TraitMethodImpl => todo!(),
        }
    }
}

impl VecMapEntry<CustomIdentifier> for CallFormDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl CallFormDecl {
    pub fn nargs(&self) -> u8 {
        (self.parameters.len() + 1).try_into().unwrap()
    }

    pub fn instantiate(&self, instantiator: &InstantiationContext) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_liason: self.this_liason,
            parameters: self
                .parameters
                .iter()
                .map(|input| input.instantiate(instantiator))
                .collect(),
            output: self.output.instantiate(instantiator),
            spatial_parameters: Default::default(),
            kind: self.kind.instantiate(instantiator),
            is_lazy: self.is_lazy,
        })
    }
    pub fn from_ast(ast: &Ast, kind: MethodKind) -> Arc<Self> {
        match ast.variant {
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                spatial_parameters: ref generic_parameters,
                ref parameters,
                output_ty,
                output_liason,
                opt_this_liason,
            } => Arc::new(CallFormDecl {
                ident: ident.ident,
                parameters: parameters.map(|input_placeholder| input_placeholder.into()),
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty.route,
                },
                this_liason: opt_this_liason.unwrap(),
                spatial_parameters: generic_parameters.clone(),
                kind,
                is_lazy: paradigm.is_lazy(),
            }),
            _ => panic!(),
        }
    }
}

pub(crate) fn call_form_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferResultArc<CallFormDecl> {
    match route.kind {
        EntityRouteKind::Root { ident } => todo!(),
        EntityRouteKind::Package { main, ident } => todo!(),
        EntityRouteKind::Child { parent, ident } => {
            let ty_decl = derived_unwrap!(db.ty_decl(parent));
            match derived_not_none!(ty_decl
                .ty_members
                .iter()
                .find(|member| member.key() == ident))?
            {
                TyMemberDecl::Field(_) => todo!(),
                TyMemberDecl::Method(method) => Ok(method.clone()),
                TyMemberDecl::Call(_) => todo!(),
            }
        }
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
            let ty_decl = derived_unwrap!(db.ty_decl(ty));
            match derived_not_none!(ty_decl.trai_member_impl(trai, ident))? {
                TraitMemberImplDecl::Method(method) => Ok(method.clone()),
                TraitMemberImplDecl::AssociatedType { ident, ty } => todo!(),
                TraitMemberImplDecl::Call {} => todo!(),
                TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
            }
        }
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
    }
}
