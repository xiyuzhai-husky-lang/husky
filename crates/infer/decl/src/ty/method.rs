use crate::*;
use atom::{symbol_proxy::Symbol, *};
use fold::LocalStack;
use implement::{Implementable, Implementor};
use map_collect::MapCollect;
use static_decl::StaticMethodDecl;
use vec_dict::HasKey;
use vm::InputContract;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDecl {
    pub ident: CustomIdentifier,
    pub this_contract: InputContract,
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
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
            output: instantiator
                .instantiate_entity_route(self.output)
                .as_scope(),
            generic_placeholders: Default::default(),
        })
    }

    pub fn implement(&self, implementor: &Implementor) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            this_contract: self.this_contract,
            inputs: self.inputs.map(|input| input.implement(implementor)),
            output: self.output.implement(implementor),
            generic_placeholders: self.generic_placeholders.clone(),
        })
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        decl: &StaticMethodDecl,
        this_ty: EntityRoutePtr,
        symbols: &LocalStack<Symbol>,
    ) -> Arc<Self> {
        let output = parse_ty(
            SymbolProxy {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: Some(this_ty),
                symbols,
            },
            &db.tokenize(decl.output_ty),
        )
        .unwrap();
        Arc::new(Self {
            ident: db.intern_word(decl.name).custom(),
            this_contract: decl.this_contract,
            inputs: decl
                .inputs
                .map(|input| InputDecl::from_static(db, input, Some(this_ty), symbols)),
            output,
            generic_placeholders: decl.generic_placeholders.map(|static_generic_placeholder| {
                GenericPlaceholder::from_static(db.upcast(), static_generic_placeholder)
            }),
        })
    }

    pub fn from_ast(method_defn_head: &MethodDefnHead) -> Arc<Self> {
        Arc::new(MethodDecl {
            ident: method_defn_head.ident,
            inputs: method_defn_head
                .input_placeholders
                .map(|input_placeholder| input_placeholder.into()),
            output: method_defn_head.output.route,
            this_contract: method_defn_head.this_contract,
            generic_placeholders: method_defn_head.generic_placeholders.clone(),
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
