use husky_opn_syntax::PrefixOpr;
use husky_word::RootIdentifier;
use std::ops::Not;
use vm::*;

pub fn resolve_primitive_prefix_opr_linkage(opr: PrefixOpr, lopd_ty: RootIdentifier) -> __Linkage {
    match opr {
        PrefixOpr::Minus => todo!(),
        PrefixOpr::Not => match lopd_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => todo!(),
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => {
                transfer_linkage!(|_, args| (0 == args[0].downcast_b32()).to_register(), none)
            }
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => {
                transfer_linkage!(|_, args|(!args[0].downcast_bool()).to_register(), some bool::not)
            }
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::Vec => todo!(),
            RootIdentifier::Tuple => todo!(),
            RootIdentifier::Debug => todo!(),
            RootIdentifier::Std => todo!(),
            RootIdentifier::Core => todo!(),
            RootIdentifier::Mor => todo!(),
            RootIdentifier::Fp => todo!(),
            RootIdentifier::Fn => todo!(),
            RootIdentifier::FnMut => todo!(),
            RootIdentifier::FnOnce => todo!(),
            RootIdentifier::Array => todo!(),
            RootIdentifier::Domains => todo!(),
            RootIdentifier::DatasetType => todo!(),
            RootIdentifier::VisualType => todo!(),
            RootIdentifier::TypeType => todo!(),
            RootIdentifier::TraitType => todo!(),
            RootIdentifier::ModuleType => todo!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
            RootIdentifier::Ref => todo!(),
            RootIdentifier::Option => todo!(),
        },
        PrefixOpr::BitNot => match lopd_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => todo!(),
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => {
                transfer_linkage!(|_, args| (!args[0].downcast_b32()).to_register(), some u32::not)
            }
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => todo!(),
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::Vec => todo!(),
            RootIdentifier::Tuple => todo!(),
            RootIdentifier::Debug => todo!(),
            RootIdentifier::Std => todo!(),
            RootIdentifier::Core => todo!(),
            RootIdentifier::Mor => todo!(),
            RootIdentifier::Fp => todo!(),
            RootIdentifier::Fn => todo!(),
            RootIdentifier::FnMut => todo!(),
            RootIdentifier::FnOnce => todo!(),
            RootIdentifier::Array => todo!(),
            RootIdentifier::Domains => todo!(),
            RootIdentifier::DatasetType => todo!(),
            RootIdentifier::VisualType => todo!(),
            RootIdentifier::TypeType => todo!(),
            RootIdentifier::TraitType => todo!(),
            RootIdentifier::ModuleType => todo!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
            RootIdentifier::Ref => todo!(),
            RootIdentifier::Option => todo!(),
        },
        PrefixOpr::Shared => todo!(),
        PrefixOpr::Move => todo!(),
    }
}
