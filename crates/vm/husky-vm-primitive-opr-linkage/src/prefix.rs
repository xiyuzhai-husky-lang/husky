use super::*;
use husky_opn_syntax::PrefixOpr;
use husky_vm_interface::*;
use husky_word::RootIdentifier;
use std::ops::Not;

pub fn resolve_primitive_prefix_opr_linkage(opr: PrefixOpr, this_ty: RootIdentifier) -> __Linkage {
    match opr {
        PrefixOpr::Minus => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => {
                transfer_linkage!(|args, _| (-args[0].downcast_i32()).to_register(), none)
            }
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => {
                transfer_linkage!(|args, _| (-args[0].downcast_f32()).to_register(), none)
            }
            RootIdentifier::F64 => {
                transfer_linkage!(|args, _| (-args[0].downcast_f64()).to_register(), none)
            }
            RootIdentifier::B32 => todo!(),
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => todo!(),
            _ => panic!(),
        },
        PrefixOpr::Not => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => todo!(),
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => {
                transfer_linkage!(|args, _| (0 == args[0].downcast_b32()).to_register(), none)
            }
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => {
                transfer_linkage!(
                    |args, _|(!args[0].downcast_bool()).to_register(),
                    some base bool::not as fn(bool) -> bool
                )
            }
            _ => panic!(),
        },
        PrefixOpr::BitNot => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => todo!(),
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => {
                transfer_linkage!(
                    |args, _| (!args[0].downcast_b32()).to_register(),
                    some base u32::not as fn(u32) -> u32
                )
            }
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => todo!(),
            _ => panic!(),
        },
        PrefixOpr::Shared => todo!(),
        PrefixOpr::Move => todo!(),
    }
}
