use crate::*;
use atom::{
    symbol::{Symbol, SymbolContextKind},
    *,
};
use fold::LocalStack;
use implement::{Implementable, Implementor};
use map_collect::MapCollect;
use print_utils::p;
use vec_map::HasKey;
use vm::InputContract;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDecl {
    pub ident: CustomIdentifier,
    pub this_contract: InputContract,
    pub inputs: Vec<InputDecl>,
    pub output: OutputDecl,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
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
                    .as_entity_route(),
            },
        }
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_kind: MethodStaticDefnKind,
        symbol_context: &SymbolContext,
    ) -> Self {
        match static_kind {
            MethodStaticDefnKind::TypeMethod { .. } => Self::Type,
            MethodStaticDefnKind::TraitMethod { .. } => {
                // opt_this_ty,
                Self::Trait {
                    trai: symbol_context.trai(),
                }
            }
            MethodStaticDefnKind::TraitMethodImpl { opt_source } => todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for MethodDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl MethodDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_contract: self.this_contract,
            inputs: self
                .inputs
                .iter()
                .map(|input| input.instantiate(instantiator))
                .collect(),
            output: self.output.instantiate(instantiator),
            generic_placeholders: Default::default(),
            kind: self.kind.instantiate(instantiator),
        })
    }

    pub fn implement(&self, implementor: &Implementor) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_contract: self.this_contract,
            inputs: self.inputs.map(|input| input.implement(implementor)),
            output: self.output.implement(implementor),
            generic_placeholders: self.generic_placeholders.clone(),
            kind: self.kind,
        })
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        defn: &EntityStaticDefn,
        symbol_context: &SymbolContext,
    ) -> Arc<Self> {
        match defn.variant {
            EntityStaticDefnVariant::Method {
                this_contract,
                input_placeholders: inputs,
                output_ty,
                output_contract,
                generic_placeholders,
                kind,
            } => {
                let output_ty = parse_ty(symbol_context, &db.tokenize(output_ty)).unwrap();
                Arc::new(Self {
                    ident: db.intern_word(defn.name).custom(),
                    this_contract: this_contract,
                    inputs: inputs.map(|input| InputDecl::from_static(db, input, symbol_context)),
                    output: OutputDecl {
                        contract: output_contract,
                        ty: output_ty,
                    },
                    generic_placeholders: generic_placeholders.map(|static_generic_placeholder| {
                        GenericPlaceholder::from_static(db.upcast(), static_generic_placeholder)
                    }),
                    kind: MethodKind::from_static(db, kind, symbol_context),
                })
            }
            _ => panic!(""),
        }
    }

    pub fn from_ast(method_defn_head: &TypeMethodDefnHead, kind: MethodKind) -> Arc<Self> {
        Arc::new(MethodDecl {
            ident: method_defn_head.ident.ident,
            inputs: method_defn_head
                .input_placeholders
                .map(|input_placeholder| input_placeholder.into()),
            output: OutputDecl {
                contract: method_defn_head.output_contract,
                ty: method_defn_head.output_ty.route,
            },
            this_contract: method_defn_head.this_contract,
            generic_placeholders: method_defn_head.generic_placeholders.clone(),
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
            let ty_decl = derived_ok!(db.ty_decl(parent));
            match derived_not_none!(ty_decl
                .ty_members
                .iter()
                .find(|member| member.key() == ident))?
            {
                TyMemberDecl::Field(_) => todo!(),
                TyMemberDecl::Method(method) => Ok(method.clone()),
                TyMemberDecl::Call => todo!(),
            }
        }
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
            let ty_decl = derived_ok!(db.ty_decl(ty));
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
