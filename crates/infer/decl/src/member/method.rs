use crate::*;
use atom::{
    context::{AtomContextKind, Symbol},
    *,
};
use fold::LocalStack;
use implement::{Implementable, Implementor};
use map_collect::MapCollect;
use print_utils::p;
use vec_map::VecMapEntry;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDecl {
    pub ident: CustomIdentifier,
    pub this_liason: ParameterLiason,
    pub parameters: Vec<ParameterDecl>,
    pub output: OutputDecl,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub kind: MethodKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodKind {
    Type,
    Trait { trai: EntityRoutePtr },
}

impl MethodKind {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            MethodKind::Type => MethodKind::Type,
            MethodKind::Trait { trai } => MethodKind::Trait {
                trai: instantiator
                    .instantiate_entity_route(*trai)
                    .take_entity_route(),
            },
        }
    }

    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        method_variant: &MethodStaticDefnVariant,
    ) -> Self {
        match method_variant {
            MethodStaticDefnVariant::TypeMethod { .. } => Self::Type,
            MethodStaticDefnVariant::TraitMethod { .. } => {
                // opt_this_ty,
                Self::Trait {
                    trai: symbol_context.trai(),
                }
            }
            MethodStaticDefnVariant::TraitMethodImpl { opt_source } => todo!(),
        }
    }
}

impl VecMapEntry<CustomIdentifier> for MethodDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl MethodDecl {
    pub fn nargs(&self) -> u8 {
        (self.parameters.len() + 1).try_into().unwrap()
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_liason: self.this_liason,
            parameters: self
                .parameters
                .iter()
                .map(|input| input.instantiate(instantiator))
                .collect(),
            output: self.output.instantiate(instantiator),
            generic_parameters: Default::default(),
            kind: self.kind.instantiate(instantiator),
        })
    }

    pub fn implement(&self, implementor: &Implementor) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_liason: self.this_liason,
            parameters: self.parameters.map(|input| input.implement(implementor)),
            output: self.output.implement(implementor),
            generic_parameters: self.generic_parameters.clone(),
            kind: self.kind,
        })
    }

    pub fn from_static(symbol_context: &mut dyn AtomContext, defn: &EntityStaticDefn) -> Arc<Self> {
        match defn.variant {
            EntityStaticDefnVariant::Method {
                this_liason: this_contract,
                parameters: inputs,
                output_ty,
                output_liason,
                generic_parameters: generic_parameters,
                ref kind,
            } => {
                let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
                Arc::new(Self {
                    ident: symbol_context
                        .entity_syntax_db()
                        .intern_word(defn.name)
                        .custom(),
                    this_liason: this_contract,
                    parameters: inputs
                        .map(|input| ParameterDecl::from_static(symbol_context, input)),
                    output: OutputDecl {
                        liason: output_liason,
                        ty: output_ty,
                    },
                    generic_parameters: generic_parameters.map(|static_generic_placeholder| {
                        SpatialParameter::from_static(
                            symbol_context.entity_syntax_db(),
                            static_generic_placeholder,
                        )
                    }),
                    kind: MethodKind::from_static(symbol_context, kind),
                })
            }
            _ => panic!(""),
        }
    }

    pub fn from_ast(method_defn_head: &CallableDefnHead, kind: MethodKind) -> Arc<Self> {
        Arc::new(MethodDecl {
            ident: method_defn_head.ident.ident,
            parameters: method_defn_head
                .parameters
                .map(|input_placeholder| input_placeholder.into()),
            output: OutputDecl {
                liason: method_defn_head.output_liason,
                ty: method_defn_head.output_ty.route,
            },
            this_liason: method_defn_head.opt_this_liason.unwrap(),
            generic_parameters: method_defn_head.generic_parameters.clone(),
            kind,
        })
    }
}

pub(crate) fn method_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferResultArc<MethodDecl> {
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
