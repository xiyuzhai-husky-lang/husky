use super::*;
use husky_opn_syntax::PrefixOpr;
use husky_vm_interface::*;
use husky_word::RootBuiltinIdentifier;
use std::ops::Not;

pub fn resolve_primitive_prefix_opr_linkage(
    opr: PrefixOpr,
    this_ty: RootBuiltinIdentifier,
) -> __Linkage {
    match opr {
        PrefixOpr::Minus => match this_ty {
            RootBuiltinIdentifier::Void => todo!(),
            RootBuiltinIdentifier::I32 => {
                transfer_linkage!(|args, _| (-args[0].downcast_i32()).to_register(), none)
            }
            RootBuiltinIdentifier::I64 => todo!(),
            RootBuiltinIdentifier::F32 => {
                transfer_linkage!(|args, _| (-args[0].downcast_f32()).to_register(), none)
            }
            RootBuiltinIdentifier::F64 => {
                transfer_linkage!(|args, _| (-args[0].downcast_f64()).to_register(), none)
            }
            RootBuiltinIdentifier::B32 => todo!(),
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => todo!(),
            _ => panic!(),
        },
        PrefixOpr::Not => match this_ty {
            RootBuiltinIdentifier::I32 => {
                transfer_linkage!(|args, _| (0 == args[0].downcast_i32()).to_register(), none)
            }
            RootBuiltinIdentifier::I64 => {
                transfer_linkage!(|args, _| (0 == args[0].downcast_i64()).to_register(), none)
            }
            RootBuiltinIdentifier::F32 => {
                transfer_linkage!(
                    |args, _| (0.0 == args[0].downcast_f32()).to_register(),
                    none
                )
            }
            RootBuiltinIdentifier::F64 => {
                transfer_linkage!(
                    |args, _| (0.0 == args[0].downcast_f64()).to_register(),
                    none
                )
            }
            RootBuiltinIdentifier::B32 => {
                transfer_linkage!(|args, _| (0 == args[0].downcast_b32()).to_register(), none)
            }
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => {
                transfer_linkage!(
                    |args, _|(!args[0].downcast_bool()).to_register(),
                    some base bool::not as fn(bool) -> bool
                )
            }
            _ => panic!(),
        },
        PrefixOpr::BitNot => match this_ty {
            RootBuiltinIdentifier::Void => todo!(),
            RootBuiltinIdentifier::I32 => todo!(),
            RootBuiltinIdentifier::I64 => todo!(),
            RootBuiltinIdentifier::F32 => todo!(),
            RootBuiltinIdentifier::F64 => todo!(),
            RootBuiltinIdentifier::B32 => {
                transfer_linkage!(
                    |args, _| (!args[0].downcast_b32()).to_register(),
                    some base u32::not as fn(u32) -> u32
                )
            }
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => todo!(),
            _ => panic!(),
        },
        PrefixOpr::Shared => todo!(),
        PrefixOpr::Move => todo!(),
    }
}
