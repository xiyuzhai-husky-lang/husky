use super::*;
use husky_entity_route::EntityRouteItd;
use husky_opn_semantics::EagerSuffixOpr;
use husky_vm_interface::*;
use husky_word::RootBuiltinIdentifier;

pub fn resolve_primitive_suffix_opr_linkage(
    opr: &EagerSuffixOpr,
    this_ty: RootBuiltinIdentifier,
) -> __Linkage {
    match opr {
        EagerSuffixOpr::Incr => match this_ty {
            RootBuiltinIdentifier::Void => todo!(),
            RootBuiltinIdentifier::I32 => {
                transfer_linkage!(
                    |args, _| {
                        unsafe { *args[0].downcast_temp_mut::<i32>(&__I32_VTABLE) += 1 }
                            .to_register()
                    },
                    none
                )
            }
            RootBuiltinIdentifier::I64 => {
                transfer_linkage!(
                    |args, _| {
                        unsafe { *args[0].downcast_temp_mut::<i64>(&__I64_VTABLE) += 1 }
                            .to_register()
                    },
                    none
                )
            }
            RootBuiltinIdentifier::F32 => todo!(),
            RootBuiltinIdentifier::F64 => todo!(),
            RootBuiltinIdentifier::B32 => todo!(),
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => todo!(),
            RootBuiltinIdentifier::True => todo!(),
            RootBuiltinIdentifier::False => todo!(),
            RootBuiltinIdentifier::Vec => todo!(),
            RootBuiltinIdentifier::Tuple => todo!(),
            RootBuiltinIdentifier::Debug => todo!(),
            RootBuiltinIdentifier::Std => todo!(),
            RootBuiltinIdentifier::Core => todo!(),
            RootBuiltinIdentifier::Mor => todo!(),
            RootBuiltinIdentifier::ThickFp => todo!(),
            RootBuiltinIdentifier::Fn => todo!(),
            RootBuiltinIdentifier::FnMut => todo!(),
            RootBuiltinIdentifier::FnOnce => todo!(),
            RootBuiltinIdentifier::Array => todo!(),
            RootBuiltinIdentifier::Domains => todo!(),
            RootBuiltinIdentifier::DatasetType => todo!(),
            RootBuiltinIdentifier::VisualType => todo!(),
            RootBuiltinIdentifier::TypeType => todo!(),
            RootBuiltinIdentifier::Trait => todo!(),
            RootBuiltinIdentifier::Module => todo!(),
            RootBuiltinIdentifier::CloneTrait => todo!(),
            RootBuiltinIdentifier::CopyTrait => todo!(),
            RootBuiltinIdentifier::PartialEqTrait => todo!(),
            RootBuiltinIdentifier::EqTrait => todo!(),
            RootBuiltinIdentifier::Ref => todo!(),
            RootBuiltinIdentifier::RefMut => todo!(),
            RootBuiltinIdentifier::Option => todo!(),
        },
        EagerSuffixOpr::Decr => match this_ty {
            RootBuiltinIdentifier::Void => todo!(),
            RootBuiltinIdentifier::I32 => {
                transfer_linkage!(
                    |args, _| {
                        unsafe { *args[0].downcast_temp_mut::<i32>(&__I32_VTABLE) -= 1 }
                            .to_register()
                    },
                    none
                )
            }
            RootBuiltinIdentifier::I64 => {
                transfer_linkage!(
                    |args, _| {
                        unsafe { *args[0].downcast_temp_mut::<i64>(&__I64_VTABLE) -= 1 }
                            .to_register()
                    },
                    none
                )
            }
            RootBuiltinIdentifier::F32 => todo!(),
            RootBuiltinIdentifier::F64 => todo!(),
            RootBuiltinIdentifier::B32 => todo!(),
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => todo!(),
            RootBuiltinIdentifier::True => todo!(),
            RootBuiltinIdentifier::False => todo!(),
            RootBuiltinIdentifier::Vec => todo!(),
            RootBuiltinIdentifier::Tuple => todo!(),
            RootBuiltinIdentifier::Debug => todo!(),
            RootBuiltinIdentifier::Std => todo!(),
            RootBuiltinIdentifier::Core => todo!(),
            RootBuiltinIdentifier::Mor => todo!(),
            RootBuiltinIdentifier::ThickFp => todo!(),
            RootBuiltinIdentifier::Fn => todo!(),
            RootBuiltinIdentifier::FnMut => todo!(),
            RootBuiltinIdentifier::FnOnce => todo!(),
            RootBuiltinIdentifier::Array => todo!(),
            RootBuiltinIdentifier::Domains => todo!(),
            RootBuiltinIdentifier::DatasetType => todo!(),
            RootBuiltinIdentifier::VisualType => todo!(),
            RootBuiltinIdentifier::TypeType => todo!(),
            RootBuiltinIdentifier::Trait => todo!(),
            RootBuiltinIdentifier::Module => todo!(),
            RootBuiltinIdentifier::CloneTrait => todo!(),
            RootBuiltinIdentifier::CopyTrait => todo!(),
            RootBuiltinIdentifier::PartialEqTrait => todo!(),
            RootBuiltinIdentifier::EqTrait => todo!(),
            RootBuiltinIdentifier::Ref => todo!(),
            RootBuiltinIdentifier::RefMut => todo!(),
            RootBuiltinIdentifier::Option => todo!(),
        },
        EagerSuffixOpr::AsTy(as_ty) => match this_ty {
            RootBuiltinIdentifier::Void => todo!(),
            RootBuiltinIdentifier::I32 => match as_ty.route {
                EntityRouteItd::Root(root_identifier) => match root_identifier {
                    RootBuiltinIdentifier::Void => todo!(),
                    RootBuiltinIdentifier::I32 => todo!(),
                    RootBuiltinIdentifier::I64 => todo!(),
                    RootBuiltinIdentifier::F32 => transfer_linkage!(
                        |args, _| (args[0].downcast_i32() as f32).to_register(),
                        none
                    ),
                    RootBuiltinIdentifier::F64 => transfer_linkage!(
                        |args, _| (args[0].downcast_i32() as u64).to_register(),
                        none
                    ),
                    RootBuiltinIdentifier::B32 => transfer_linkage!(
                        |args, _| (args[0].downcast_i32() as u32).to_register(),
                        none
                    ),
                    RootBuiltinIdentifier::B64 => todo!(),
                    RootBuiltinIdentifier::Bool => todo!(),
                    RootBuiltinIdentifier::True => todo!(),
                    RootBuiltinIdentifier::False => todo!(),
                    RootBuiltinIdentifier::Vec => todo!(),
                    RootBuiltinIdentifier::Tuple => todo!(),
                    RootBuiltinIdentifier::Debug => todo!(),
                    RootBuiltinIdentifier::Std => todo!(),
                    RootBuiltinIdentifier::Core => todo!(),
                    RootBuiltinIdentifier::Mor => todo!(),
                    RootBuiltinIdentifier::ThickFp => todo!(),
                    RootBuiltinIdentifier::Fn => todo!(),
                    RootBuiltinIdentifier::FnMut => todo!(),
                    RootBuiltinIdentifier::FnOnce => todo!(),
                    RootBuiltinIdentifier::Array => todo!(),
                    RootBuiltinIdentifier::Domains => todo!(),
                    RootBuiltinIdentifier::DatasetType => todo!(),
                    RootBuiltinIdentifier::VisualType => todo!(),
                    RootBuiltinIdentifier::TypeType => todo!(),
                    RootBuiltinIdentifier::Trait => todo!(),
                    RootBuiltinIdentifier::Module => todo!(),
                    RootBuiltinIdentifier::CloneTrait => todo!(),
                    RootBuiltinIdentifier::CopyTrait => todo!(),
                    RootBuiltinIdentifier::PartialEqTrait => todo!(),
                    RootBuiltinIdentifier::EqTrait => todo!(),
                    RootBuiltinIdentifier::Ref => todo!(),
                    RootBuiltinIdentifier::RefMut => todo!(),
                    RootBuiltinIdentifier::Option => todo!(),
                },
                EntityRouteItd::Custom(_) => todo!(),
            },
            RootBuiltinIdentifier::I64 => todo!(),
            RootBuiltinIdentifier::F32 => todo!(),
            RootBuiltinIdentifier::F64 => todo!(),
            RootBuiltinIdentifier::B32 => match as_ty.route {
                EntityRouteItd::Root(root_identifier) => match root_identifier {
                    RootBuiltinIdentifier::Void => todo!(),
                    RootBuiltinIdentifier::I32 => transfer_linkage!(
                        |args, _| (args[0].downcast_b32() as i32).to_register(),
                        none
                    ),
                    RootBuiltinIdentifier::I64 => todo!(),
                    RootBuiltinIdentifier::F32 => todo!(),
                    RootBuiltinIdentifier::F64 => todo!(),
                    RootBuiltinIdentifier::B32 => todo!(),
                    RootBuiltinIdentifier::B64 => todo!(),
                    RootBuiltinIdentifier::Bool => todo!(),
                    RootBuiltinIdentifier::True => todo!(),
                    RootBuiltinIdentifier::False => todo!(),
                    RootBuiltinIdentifier::Vec => todo!(),
                    RootBuiltinIdentifier::Tuple => todo!(),
                    RootBuiltinIdentifier::Debug => todo!(),
                    RootBuiltinIdentifier::Std => todo!(),
                    RootBuiltinIdentifier::Core => todo!(),
                    RootBuiltinIdentifier::Mor => todo!(),
                    RootBuiltinIdentifier::ThickFp => todo!(),
                    RootBuiltinIdentifier::Fn => todo!(),
                    RootBuiltinIdentifier::FnMut => todo!(),
                    RootBuiltinIdentifier::FnOnce => todo!(),
                    RootBuiltinIdentifier::Array => todo!(),
                    RootBuiltinIdentifier::Domains => todo!(),
                    RootBuiltinIdentifier::DatasetType => todo!(),
                    RootBuiltinIdentifier::VisualType => todo!(),
                    RootBuiltinIdentifier::TypeType => todo!(),
                    RootBuiltinIdentifier::Trait => todo!(),
                    RootBuiltinIdentifier::Module => todo!(),
                    RootBuiltinIdentifier::CloneTrait => todo!(),
                    RootBuiltinIdentifier::CopyTrait => todo!(),
                    RootBuiltinIdentifier::PartialEqTrait => todo!(),
                    RootBuiltinIdentifier::EqTrait => todo!(),
                    RootBuiltinIdentifier::Ref => todo!(),
                    RootBuiltinIdentifier::RefMut => todo!(),
                    RootBuiltinIdentifier::Option => todo!(),
                },
                EntityRouteItd::Custom(_) => todo!(),
            },
            RootBuiltinIdentifier::B64 => todo!(),
            RootBuiltinIdentifier::Bool => todo!(),
            RootBuiltinIdentifier::True => todo!(),
            RootBuiltinIdentifier::False => todo!(),
            RootBuiltinIdentifier::Vec => todo!(),
            RootBuiltinIdentifier::Tuple => todo!(),
            RootBuiltinIdentifier::Debug => todo!(),
            RootBuiltinIdentifier::Std => todo!(),
            RootBuiltinIdentifier::Core => todo!(),
            RootBuiltinIdentifier::Mor => todo!(),
            RootBuiltinIdentifier::ThickFp => todo!(),
            RootBuiltinIdentifier::Fn => todo!(),
            RootBuiltinIdentifier::FnMut => todo!(),
            RootBuiltinIdentifier::FnOnce => todo!(),
            RootBuiltinIdentifier::Array => todo!(),
            RootBuiltinIdentifier::Domains => todo!(),
            RootBuiltinIdentifier::DatasetType => todo!(),
            RootBuiltinIdentifier::VisualType => todo!(),
            RootBuiltinIdentifier::TypeType => todo!(),
            RootBuiltinIdentifier::Trait => todo!(),
            RootBuiltinIdentifier::Module => todo!(),
            RootBuiltinIdentifier::CloneTrait => todo!(),
            RootBuiltinIdentifier::CopyTrait => todo!(),
            RootBuiltinIdentifier::PartialEqTrait => todo!(),
            RootBuiltinIdentifier::EqTrait => todo!(),
            RootBuiltinIdentifier::Ref => todo!(),
            RootBuiltinIdentifier::RefMut => todo!(),
            RootBuiltinIdentifier::Option => todo!(),
        },
        EagerSuffixOpr::BePattern(_) => panic!(),
        EagerSuffixOpr::Unveil => todo!(),
    }
}
