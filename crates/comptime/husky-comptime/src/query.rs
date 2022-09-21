use std::sync::Arc;

use husky_entity_kind::TyKind;
use husky_entity_route::EntityRoutePtr;
use husky_file::FilePtr;
use husky_linkage_table::ResolveLinkage;
use husky_package_semantics::PackageQueryGroup;
use husky_trace_protocol::Label;
use husky_vm::{__Register, __RegisterDataKind, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use husky_word::RootIdentifier;
use infer_decl::TyDecl;

use crate::{
    utils::{
        __RegisterDowncastResult, convert_enum_register_to_label, convert_i32_register_to_label,
    },
    HuskyComptime,
};

pub trait ComptimeQueryGroup: PackageQueryGroup + ResolveLinkage {
    fn target_entrance(&self) -> FilePtr {
        self.opt_target_entrance().unwrap()
    }
    fn register_to_label_converter(
        &self,
    ) -> for<'eval> fn(&__Register<'eval>) -> __RegisterDowncastResult<Label> {
        let target_output_ty = self.target_output_ty().unwrap();
        let target_output_ty_intrinsic = target_output_ty.intrinsic();

        if target_output_ty_intrinsic == RootIdentifier::I32.into() {
            convert_i32_register_to_label
        } else {
            let target_output_ty_intrinsic_decl = self.ty_decl(target_output_ty_intrinsic).unwrap();
            match target_output_ty_intrinsic_decl.ty_kind {
                TyKind::Enum => convert_enum_register_to_label,
                TyKind::Record => todo!(),
                TyKind::Struct => todo!(),
                TyKind::Primitive => todo!(),
                TyKind::Vec => todo!(),
                TyKind::Slice => todo!(),
                TyKind::CyclicSlice => todo!(),
                TyKind::Array => todo!(),
                TyKind::Tuple => todo!(),
                TyKind::Mor => todo!(),
                TyKind::ThickFp => todo!(),
                TyKind::AssociatedAny => todo!(),
                TyKind::ThisAny => todo!(),
                TyKind::TargetOutputAny => todo!(),
                TyKind::SpatialPlaceholderAny => todo!(),
                TyKind::BoxAny => todo!(),
                TyKind::HigherKind => todo!(),
                TyKind::Ref => todo!(),
                TyKind::Option => todo!(),
            }
        }
    }

    // ad hoc loc
    fn print_short<'eval>(&self, value: &__Register<'eval>, ty: EntityRoutePtr) -> String {
        if value.data_kind() == __RegisterDataKind::SomeNone {
            if unsafe { value.data().as_number_of_somes } > 0 {
                todo!()
            } else {
                return "none".to_owned();
            }
        }
        let intrinsic_ty = ty.intrinsic();
        match intrinsic_ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void => todo!(),
                RootIdentifier::I32 => match value.data_kind() {
                    __RegisterDataKind::Moved => todo!(),
                    __RegisterDataKind::SomeNone => todo!(),
                    __RegisterDataKind::Unreturned => "unreturned".to_string(),
                    _ => format!("{}", value.downcast_i32()),
                },
                RootIdentifier::I64 => todo!(),
                RootIdentifier::F32 => match value.data_kind() {
                    __RegisterDataKind::Moved => todo!(),
                    __RegisterDataKind::SomeNone => todo!(),
                    __RegisterDataKind::Unreturned => "unreturned".to_string(),
                    _ => format!("{}", value.downcast_f32()),
                },
                RootIdentifier::F64 => todo!(),
                RootIdentifier::B32 => todo!(),
                RootIdentifier::B64 => todo!(),
                RootIdentifier::Bool => format!("{}", value.downcast_bool()),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::ThickFp => todo!(),
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
                RootIdentifier::RefMut => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                let ty_decl: Arc<TyDecl> = self.ty_decl(intrinsic_ty).unwrap();
                match ty_decl.ty_kind {
                    TyKind::Enum => {
                        let value: &__VirtualEnum = value.downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
                        let enum_variant_decl = &ty_decl.variants.data()[value.kind_idx as usize];
                        format!("{}::{}", intrinsic_ty.ident(), enum_variant_decl.ident)
                    }
                    TyKind::Record => todo!(),
                    TyKind::Struct => "{ ... }".to_string(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => "[ ... ]".to_string(),
                    TyKind::Array => todo!(),
                    TyKind::Slice => todo!(),
                    TyKind::CyclicSlice => "[ ... ]".to_string(),
                    TyKind::Tuple => todo!(),
                    TyKind::Mor => todo!(),
                    TyKind::ThickFp => todo!(),
                    TyKind::AssociatedAny => todo!(),
                    TyKind::ThisAny => todo!(),
                    TyKind::SpatialPlaceholderAny => todo!(),
                    TyKind::BoxAny => todo!(),
                    TyKind::HigherKind => todo!(),
                    TyKind::Ref => todo!(),
                    TyKind::Option => todo!(),
                    TyKind::TargetOutputAny => todo!(),
                }
            }
        }
    }
}

impl ComptimeQueryGroup for HuskyComptime {}
