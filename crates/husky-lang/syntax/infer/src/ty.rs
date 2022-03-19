mod sheet;
mod signature;

pub(crate) use sheet::*;
pub(crate) use signature::*;

use ast::AstIter;
use scope::StaticScopeSignature;
use syntax_types::{MembVarSignature, RawEnumVariantKind};
use vec_map::VecMap;
use vm::{MembVarContract, VMTySignature};

use crate::*;

pub type IdentMap<T> = VecMap<CustomIdentifier, T>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Struct {
        memb_vars: IdentMap<MembVarSignature>,
    },
    Enum {
        variants: IdentMap<EnumVariantSignature>,
    },
}

impl TySignature {
    pub fn memb_var_ty(&self, ident: CustomIdentifier) -> ScopePtr {
        match self {
            TySignature::Struct { ref memb_vars } => memb_vars.get(ident).unwrap().ty,
            TySignature::Enum { ref variants } => todo!(),
        }
    }

    pub fn memb_var_signature(&self, ident: CustomIdentifier) -> &MembVarSignature {
        match self {
            TySignature::Struct { ref memb_vars } => memb_vars.get(ident).unwrap(),
            TySignature::Enum { ref variants } => todo!(),
        }
    }

    pub fn vm_ty_signature(&self) -> VMTySignature {
        match self {
            TySignature::Struct { memb_vars } => {
                let mut vm_memb_vars = IdentMap::<MembVarContract>::default();
                memb_vars.iter().for_each(|(ident, memb_var_sig)| {
                    vm_memb_vars.insert_new(*ident, memb_var_sig.contract)
                });
                VMTySignature::Struct {
                    memb_vars: vm_memb_vars,
                }
            }
            TySignature::Enum { variants } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantSignature {
    Constant,
}
