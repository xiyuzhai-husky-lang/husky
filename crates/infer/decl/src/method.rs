use crate::*;
use print_utils::p;
use static_decl::StaticMethodDecl;
use vec_map::HasKey;
use vm::{InputContract, MembAccessContract};
use word::IdentDict;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
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
        }
    }

    pub fn from_static(db: &dyn DeclQueryGroup, decl: &StaticMethodDecl) -> Self {
        let output = db.parse_ty(decl.output_ty).unwrap();
        Self {
            ident: db.intern_word(decl.name).custom(),
            this_contract: decl.this_contract,
            inputs: decl
                .inputs
                .iter()
                .map(|input| InputDecl::from_static(db, input))
                .collect(),
            output,
            generic_placeholders: decl
                .generic_placeholders
                .iter()
                .map(|static_generic_placeholder| {
                    GenericPlaceholder::from_static(db.upcast(), static_generic_placeholder)
                })
                .collect(),
        }
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
