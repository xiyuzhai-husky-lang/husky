use crate::*;
use atom::{
    symbol::{Symbol, SymbolContextKind},
    *,
};
use fold::LocalStack;
use implement::{Implementable, Implementor};
use map_collect::MapCollect;
use print_utils::p;
use static_decl::StaticMethodDecl;
use vec_dict::HasKey;
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
    Trait(EntityRoutePtr),
}

impl MethodKind {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            MethodKind::Type => MethodKind::Type,
            MethodKind::Trait(trai) => {
                MethodKind::Trait(instantiator.instantiate_entity_route(*trai).as_scope())
            }
        }
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_kind: &StaticMethodKind,
        symbol_context: &SymbolContext,
    ) -> Self {
        match static_kind {
            StaticMethodKind::TypeMethod => Self::Type,
            StaticMethodKind::TraitMethod(trai) => {
                // opt_this_ty,
                Self::Trait(symbol_context.entity_route_from_str(trai).unwrap())
            }
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
        decl: &StaticMethodDecl,
        symbol_context: &SymbolContext,
    ) -> Arc<Self> {
        // SymbolContext {
        //     opt_package_main: None,
        //     db: db.upcast(),
        //     opt_this_ty: opt_this_ty,
        //     symbols,
        //     variant: SymbolContextKind::Normal,
        // },
        let output_ty = parse_ty(symbol_context, &db.tokenize(decl.output_ty)).unwrap();
        Arc::new(Self {
            ident: db.intern_word(decl.name).custom(),
            this_contract: decl.this_contract,
            inputs: decl
                .inputs
                .map(|input| InputDecl::from_static(db, input, symbol_context)),
            output: OutputDecl {
                contract: decl.output_contract,
                ty: output_ty,
            },
            generic_placeholders: decl.generic_placeholders.map(|static_generic_placeholder| {
                GenericPlaceholder::from_static(db.upcast(), static_generic_placeholder)
            }),
            kind: MethodKind::from_static(db, &decl.kind, symbol_context),
        })
    }

    pub fn from_ast(method_defn_head: &TypeMethodDefnHead, kind: MethodKind) -> Arc<Self> {
        Arc::new(MethodDecl {
            ident: method_defn_head.ident,
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

// impl MembDecl {
//     pub fn instantiate(&self, instantiator: &Instantiator) -> MembDecl {
//         match self.variant {
//             FieldDeclVariant::Var(ref decl) => MembDecl {
//                 variant: FieldDeclVariant::Var(decl.instantiate(instantiator)),
//             },
//             FieldDeclVariant::Routine(ref decl) => MembDecl {
//                 variant: FieldDeclVariant::Routine(decl.instantiate(instantiator)),
//             },
//         }
//     }
