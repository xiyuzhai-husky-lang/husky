use husky_linkage_table::ResolveLinkage;
use husky_source_path::SourcePath;
use husky_term::Term;
use husky_trace_protocol::Label;
use husky_vm::{__Register, __RegisterDataKind};

use crate::{utils::__RegisterDowncastResult, AnalysisHost};

pub trait ComptimeQueryGroup {
    fn target_entrance(&self) -> SourcePath {
        todo!()
        // self.opt_target_entrance().unwrap()
    }
    fn register_to_label_converter(
        &self,
    ) -> for<'eval> fn(&__Register<'eval>) -> __RegisterDowncastResult<Label> {
        todo!()
        // let target_output_ty = self.target_output_ty().unwrap();
        // let target_output_ty_intrinsic = target_output_ty.intrinsic();

        // if target_output_ty_intrinsic == RootBuiltinIdentifier::I32.into() {
        //     convert_i32_register_to_label
        // } else {
        //     let target_output_ty_intrinsic_decl = self.ty_decl(target_output_ty_intrinsic).unwrap();
        //     match target_output_ty_intrinsic_decl.ty_kind {
        //         TyKind::Enum => convert_enum_register_to_label,
        //         TyKind::Record => todo!(),
        //         TyKind::Struct => todo!(),
        //         TyKind::Primitive => todo!(),
        //         TyKind::Vec => todo!(),
        //         TyKind::Slice => todo!(),
        //         TyKind::CyclicSlice => todo!(),
        //         TyKind::Array => todo!(),
        //         TyKind::Tuple => todo!(),
        //         TyKind::Mor => todo!(),
        //         TyKind::ThickFp => todo!(),
        //         TyKind::AssociatedAny => todo!(),
        //         TyKind::ThisAny => todo!(),
        //         TyKind::TargetOutputAny => todo!(),
        //         TyKind::SpatialPlaceholderAny => todo!(),
        //         TyKind::BoxAny => todo!(),
        //         TyKind::HigherKind => todo!(),
        //         TyKind::Ref => todo!(),
        //         TyKind::Option => todo!(),
        //     }
        // }
    }

    // ad hoc loc
    fn print_short<'eval>(&self, value: &__Register<'eval>, ty: Term) -> String {
        todo!()
        // if value.data_kind() == __RegisterDataKind::SomeNone {
        //     if unsafe { value.data().as_number_of_somes } > 0 {
        //         todo!()
        //     } else {
        //         return "none".to_owned();
        //     }
        // }
        // let intrinsic_ty = ty.intrinsic();
        // match intrinsic_ty {
        //     Term::Root(root_identifier) => match root_identifier {
        //         RootBuiltinIdentifier::Void => return "()".to_owned(),
        //         RootBuiltinIdentifier::I32 => match value.data_kind() {
        //             __RegisterDataKind::Moved => todo!(),
        //             __RegisterDataKind::SomeNone => todo!(),
        //             __RegisterDataKind::Unreturned => "unreturned".to_string(),
        //             _ => format!("{}", value.downcast_i32()),
        //         },
        //         RootBuiltinIdentifier::I64 => todo!(),
        //         RootBuiltinIdentifier::F32 => match value.data_kind() {
        //             __RegisterDataKind::Moved => todo!(),
        //             __RegisterDataKind::SomeNone => todo!(),
        //             __RegisterDataKind::Unreturned => "unreturned".to_string(),
        //             _ => format!("{}", value.downcast_f32()),
        //         },
        //         RootBuiltinIdentifier::F64 => todo!(),
        //         RootBuiltinIdentifier::B32 => match value.data_kind() {
        //             __RegisterDataKind::Moved => todo!(),
        //             __RegisterDataKind::SomeNone => todo!(),
        //             __RegisterDataKind::Unreturned => "unreturned".to_string(),
        //             _ => format!("{}", value.downcast_b32()),
        //         },
        //         RootBuiltinIdentifier::B64 => todo!(),
        //         RootBuiltinIdentifier::Bool => format!("{}", value.downcast_bool()),
        //         RootBuiltinIdentifier::True => "true".into(),
        //         RootBuiltinIdentifier::False => "false".into(),
        //         RootBuiltinIdentifier::Vec => todo!(),
        //         RootBuiltinIdentifier::Tuple => todo!(),
        //         RootBuiltinIdentifier::Debug => todo!(),
        //         RootBuiltinIdentifier::Std => todo!(),
        //         RootBuiltinIdentifier::Core => todo!(),
        //         RootBuiltinIdentifier::Mor => todo!(),
        //         RootBuiltinIdentifier::ThickFp => todo!(),
        //         RootBuiltinIdentifier::Fn => todo!(),
        //         RootBuiltinIdentifier::FnMut => todo!(),
        //         RootBuiltinIdentifier::FnOnce => todo!(),
        //         RootBuiltinIdentifier::Array => todo!(),
        //         RootBuiltinIdentifier::Domains => todo!(),
        //         RootBuiltinIdentifier::DatasetType => todo!(),
        //         RootBuiltinIdentifier::VisualType => todo!(),
        //         RootBuiltinIdentifier::TypeType => todo!(),
        //         RootBuiltinIdentifier::Trait => todo!(),
        //         RootBuiltinIdentifier::Module => todo!(),
        //         RootBuiltinIdentifier::CloneTrait => todo!(),
        //         RootBuiltinIdentifier::CopyTrait => todo!(),
        //         RootBuiltinIdentifier::PartialEqTrait => todo!(),
        //         RootBuiltinIdentifier::EqTrait => todo!(),
        //         RootBuiltinIdentifier::Ref => todo!(),
        //         RootBuiltinIdentifier::RefMut => todo!(),
        //         RootBuiltinIdentifier::Option => todo!(),
        //     },
        //     Term::Custom(_) => {
        //         todo!()
        //         // let ty_decl: Arc<TyDecl> = self.ty_decl(intrinsic_ty).unwrap();
        //         // match ty_decl.ty_kind {
        //         //     TyKind::Enum => {
        //         //         let value: &__VirtualEnum = value.downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
        //         //         let enum_variant_decl = &ty_decl.variants.data()[value.kind_idx as usize];
        //         //         format!("{}::{}", intrinsic_ty.ident(), enum_variant_decl.ident)
        //         //     }
        //         //     TyKind::Record => todo!(),
        //         //     TyKind::Struct => "{ ... }".to_string(),
        //         //     TyKind::Primitive => todo!(),
        //         //     TyKind::Vec => "[ ... ]".to_string(),
        //         //     TyKind::Array => todo!(),
        //         //     TyKind::Slice => todo!(),
        //         //     TyKind::CyclicSlice => "[ ... ]".to_string(),
        //         //     TyKind::Tuple => todo!(),
        //         //     TyKind::Mor => todo!(),
        //         //     TyKind::ThickFp => todo!(),
        //         //     TyKind::AssociatedAny => todo!(),
        //         //     TyKind::ThisAny => todo!(),
        //         //     TyKind::SpatialPlaceholderAny => todo!(),
        //         //     TyKind::BoxAny => todo!(),
        //         //     TyKind::HigherKind => todo!(),
        //         //     TyKind::Ref => todo!(),
        //         //     TyKind::Option => todo!(),
        //         //     TyKind::TargetOutputAny => todo!(),
        //         // }
        //     }
        // }
    }
}

impl ComptimeQueryGroup for AnalysisHost {}
