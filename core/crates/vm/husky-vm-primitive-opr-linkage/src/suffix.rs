use super::*;
use husky_entity_route::EntityRoutePtr;
use husky_opn_semantics::SuffixOpr;
use husky_vm_interface::*;
use husky_word::RootIdentifier;

pub fn resolve_primitive_suffix_opr_linkage(opr: &SuffixOpr, this_ty: RootIdentifier) -> __Linkage {
    match opr {
        SuffixOpr::Incr => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => {
                transfer_linkage!(
                    |_, args| { unsafe { *args[0].downcast_temp_mut::<i32>() += 1 }.to_register() },
                    none
                )
            }
            RootIdentifier::I64 => {
                transfer_linkage!(
                    |_, args| { unsafe { *args[0].downcast_temp_mut::<i64>() += 1 }.to_register() },
                    none
                )
            }
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => todo!(),
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
        SuffixOpr::Decr => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => {
                transfer_linkage!(
                    |_, args| { unsafe { *args[0].downcast_temp_mut::<i32>() -= 1 }.to_register() },
                    none
                )
            }
            RootIdentifier::I64 => {
                transfer_linkage!(
                    |_, args| { unsafe { *args[0].downcast_temp_mut::<i64>() -= 1 }.to_register() },
                    none
                )
            }
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => todo!(),
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
        SuffixOpr::AsTy(as_ty) => match this_ty {
            RootIdentifier::Void => todo!(),
            RootIdentifier::I32 => match as_ty.route {
                EntityRoutePtr::Root(root_identifier) => match root_identifier {
                    RootIdentifier::Void => todo!(),
                    RootIdentifier::I32 => todo!(),
                    RootIdentifier::I64 => todo!(),
                    RootIdentifier::F32 => transfer_linkage!(
                        |_, args| (args[0].downcast_i32() as f32).to_register(),
                        none
                    ),
                    RootIdentifier::F64 => transfer_linkage!(
                        |_, args| (args[0].downcast_i32() as u64).to_register(),
                        none
                    ),
                    RootIdentifier::B32 => transfer_linkage!(
                        |_, args| (args[0].downcast_i32() as u32).to_register(),
                        none
                    ),
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
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            RootIdentifier::I64 => todo!(),
            RootIdentifier::F32 => todo!(),
            RootIdentifier::F64 => todo!(),
            RootIdentifier::B32 => match as_ty.route {
                EntityRoutePtr::Root(root_identifier) => match root_identifier {
                    RootIdentifier::Void => todo!(),
                    RootIdentifier::I32 => transfer_linkage!(
                        |_, args| (args[0].downcast_b32() as i32).to_register(),
                        none
                    ),
                    RootIdentifier::I64 => todo!(),
                    RootIdentifier::F32 => todo!(),
                    RootIdentifier::F64 => todo!(),
                    RootIdentifier::B32 => todo!(),
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
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
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
        SuffixOpr::BePattern(_) => panic!(),
    }
}
