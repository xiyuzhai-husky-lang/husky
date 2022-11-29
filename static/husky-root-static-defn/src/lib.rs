#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(panic_info_message)]
pub mod __std;
mod b32;
mod clone;
pub mod domains;
mod eq;
mod etc;
mod f32;
mod i32;
mod thick_fp;
mod vec;

pub use crate::clone::*;
pub use crate::f32::*;
pub use crate::i32::*;
pub use __husky::{init::*, root::*};
pub use __std::*;
pub use b32::*;
pub use domains::*;
pub use eq::*;
pub use etc::*;
pub use husky_term::Term as __EntityRoutePtr;
pub use serde::Serialize as __Serialize;
pub use serde_json::value::Value as __JsonValue;
pub use std::sync::Arc as __Arc;
pub use thick_fp::*;
pub use vec::*;

use husky_entity_kind::{FieldKind, TyKind};
use husky_liason_semantics::{MemberModifier, OutputModifier, ParameterModifier};
use husky_static_defn::StaticParameter;
use husky_static_defn::*;
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_vm::*;
use husky_word::Identifier;

pub fn __resolve_root_defn(ident: Identifier) -> &'static EntityStaticDefn {
    todo!()
    // match ident {
    //     RootBuiltinIdentifier::Void => &VOID_TYPE_DEFN,
    //     RootBuiltinIdentifier::I32 => &I32_TYPE_DEFN,
    //     RootBuiltinIdentifier::I64 => todo!(),
    //     RootBuiltinIdentifier::F32 => &F32_TYPE_DEFN,
    //     RootBuiltinIdentifier::F64 => todo!(),
    //     RootBuiltinIdentifier::B32 => &B32_TYPE_DEFN,
    //     RootBuiltinIdentifier::B64 => &B64_TYPE_DEFN,
    //     RootBuiltinIdentifier::Bool => &BOOL_TYPE_DEFN,
    //     RootBuiltinIdentifier::True => todo!(),
    //     RootBuiltinIdentifier::False => todo!(),
    //     RootBuiltinIdentifier::Vec => &VEC_TYPE_DEFN,
    //     RootBuiltinIdentifier::Tuple => todo!(),
    //     RootBuiltinIdentifier::Debug => todo!(),
    //     RootBuiltinIdentifier::Std => &STD_DEFN,
    //     RootBuiltinIdentifier::Core => todo!(),
    //     RootBuiltinIdentifier::Mor => todo!(),
    //     RootBuiltinIdentifier::ThickFp => &FP_TYPE_DEFN,
    //     RootBuiltinIdentifier::Fn => todo!(),
    //     RootBuiltinIdentifier::FnMut => todo!(),
    //     RootBuiltinIdentifier::FnOnce => todo!(),
    //     RootBuiltinIdentifier::Array => todo!(),
    //     RootBuiltinIdentifier::Domains => &DOMAINS_DEFN,
    //     RootBuiltinIdentifier::DatasetType => &husky_datasets_static_defn::DATASET_TYPE_DEFN,
    //     RootBuiltinIdentifier::TypeType => &TYPE_TYPE_DEFN,
    //     RootBuiltinIdentifier::Trait => &TRAIT_TYPE_DEFN,
    //     RootBuiltinIdentifier::CloneTrait => &CLONE_TRAIT_DEFN,
    //     RootBuiltinIdentifier::CopyTrait => todo!(),
    //     RootBuiltinIdentifier::PartialEqTrait => todo!(),
    //     RootBuiltinIdentifier::EqTrait => todo!(),
    //     RootBuiltinIdentifier::Module => &MODULE_TYPE_DEFN,
    //     RootBuiltinIdentifier::Ref => panic!(),
    //     RootBuiltinIdentifier::Option => panic!(),
    //     RootBuiltinIdentifier::VisualType => todo!(),
    //     RootBuiltinIdentifier::RefMut => todo!(),
    // }
    // .into()
}

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: "void",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B64_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b64",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: "b64",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::B64,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "bool",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: "bool",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Bool,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
