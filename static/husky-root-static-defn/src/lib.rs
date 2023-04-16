#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(panic_info_message)]
pub mod __std;
mod clone;
pub mod domains;
mod eq;
mod etc;
mod f32;
mod i32;
mod r32;
mod thick_fp;
mod vec;

pub use crate::clone::*;
pub use crate::f32::*;
pub use crate::i32::*;
pub use __husky::{init::*, root::*};
pub use __std::*;
pub use domains::*;
pub use eq::*;
pub use etc::*;
pub use husky_ethereal_term::EtherealTerm as __EntityRoutePtr;
pub use r32::*;
pub use serde::Serialize as __Serialize;
pub use serde_json::value::Value as __JsonValue;
pub use std::sync::Arc as __Arc;
pub use thick_fp::*;
pub use vec::*;

use husky_entity_taxonomy::{FieldKind, TyKind};
use husky_liason_semantics::{MemberModifier, OutputModifier, ParameterModifier};
use husky_static_defn::StaticParameter;
use husky_static_defn::*;
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_vm::*;
use husky_word::Ident;

pub fn __resolve_root_defn(ident: Ident) -> &'static EntityStaticDefn {
    todo!()
    // match ident {
    //     RootBuiltinIdent::Void => &VOID_TYPE_DEFN,
    //     RootBuiltinIdent::I32 => &I32_TYPE_DEFN,
    //     RootBuiltinIdent::I64 => todo!(),
    //     RootBuiltinIdent::F32 => &F32_TYPE_DEFN,
    //     RootBuiltinIdent::F64 => todo!(),
    //     RootBuiltinIdent::B32 => &B32_TYPE_DEFN,
    //     RootBuiltinIdent::B64 => &B64_TYPE_DEFN,
    //     RootBuiltinIdent::Bool => &BOOL_TYPE_DEFN,
    //     RootBuiltinIdent::True => todo!(),
    //     RootBuiltinIdent::False => todo!(),
    //     RootBuiltinIdent::Vec => &VEC_TYPE_DEFN,
    //     RootBuiltinIdent::Tuple => todo!(),
    //     RootBuiltinIdent::Debug => todo!(),
    //     RootBuiltinIdent::Std => &STD_DEFN,
    //     RootBuiltinIdent::Core => todo!(),
    //     RootBuiltinIdent::Mor => todo!(),
    //     RootBuiltinIdent::ThickFp => &FP_TYPE_DEFN,
    //     RootBuiltinIdent::Fn => todo!(),
    //     RootBuiltinIdent::FnMut => todo!(),
    //     RootBuiltinIdent::FnOnce => todo!(),
    //     RootBuiltinIdent::Array => todo!(),
    //     RootBuiltinIdent::Domains => &DOMAINS_DEFN,
    //     RootBuiltinIdent::DatasetType => &husky_datasets_static_defn::DATASET_TYPE_DEFN,
    //     RootBuiltinIdent::TypeType => &TYPE_TYPE_DEFN,
    //     RootBuiltinIdent::Trait => &TRAIT_TYPE_DEFN,
    //     RootBuiltinIdent::CloneTrait => &CLONE_TRAIT_DEFN,
    //     RootBuiltinIdent::CopyTrait => todo!(),
    //     RootBuiltinIdent::PartialEqTrait => todo!(),
    //     RootBuiltinIdent::EqTrait => todo!(),
    //     RootBuiltinIdent::Module => &MODULE_TYPE_DEFN,
    //     RootBuiltinIdent::Ref => panic!(),
    //     RootBuiltinIdent::Option => panic!(),
    //     RootBuiltinIdent::VisualType => todo!(),
    //     RootBuiltinIdent::RefMut => todo!(),
    // }
    // .into()
}

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
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
    variant: EntityStaticDefnVariant::EtherealTerm {
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
    variant: EntityStaticDefnVariant::EtherealTerm {
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
