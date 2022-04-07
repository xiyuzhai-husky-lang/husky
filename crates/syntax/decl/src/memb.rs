use crate::*;
use print_utils::p;
use vm::{InputContract, MembAccessContract};
use word::IdentMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembDecl {
    pub variant: MembDeclVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembDeclVariant {
    Var(MembAccessDecl),
    Routine(MembCallDecl),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembCallDecl {
    pub this_contract: InputContract,
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
    pub generic_placeholders: IdentMap<GenericPlaceholder>,
}

impl MembCallDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            this_contract: self.this_contract,
            inputs: self
                .inputs
                .iter()
                .map(|input| input.instantiate(instantiator))
                .collect(),
            output: instantiator.instantiate_scope(self.output).as_scope(),
            generic_placeholders: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MembAccessDecl {
    pub contract: MembAccessContract,
    pub ty: EntityRoutePtr,
}

impl MembAccessDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        todo!()
    }
}

impl MembDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> MembDecl {
        match self.variant {
            MembDeclVariant::Var(ref decl) => MembDecl {
                variant: MembDeclVariant::Var(decl.instantiate(instantiator)),
            },
            MembDeclVariant::Routine(ref decl) => MembDecl {
                variant: MembDeclVariant::Routine(decl.instantiate(instantiator)),
            },
        }
    }

    pub fn from_static(db: &dyn DeclQueryGroup, variant: &StaticMembDeclVariant) -> Self {
        match variant {
            StaticMembDeclVariant::Var { ty } => todo!(),
            StaticMembDeclVariant::Routine {
                this_contract,
                inputs,
                output_ty,
                ref generic_placeholders,
            } => {
                let output = db.parse_ty(output_ty).unwrap();
                Self {
                    variant: MembDeclVariant::Routine(MembCallDecl {
                        this_contract: *this_contract,
                        inputs: inputs
                            .iter()
                            .map(|input| InputDecl::from_static(db, input))
                            .collect(),
                        output,
                        generic_placeholders: generic_placeholders
                            .iter()
                            .map(|static_generic_placeholder| {
                                (
                                    static_generic_placeholder.ident,
                                    static_generic_placeholder.into(),
                                )
                            })
                            .collect(),
                    }),
                }
            }
        }
    }
}
