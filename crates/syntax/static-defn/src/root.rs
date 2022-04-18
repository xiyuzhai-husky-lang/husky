mod static_std;

pub use static_std::*;

use crate::*;
use entity_kind::TyKind;
use visual_syntax::TRIVIAL_VISUALIZER;

pub static CLONE_TRAIT_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Trait(&CLONE_TRAIT_DECL),
};

pub static VOID_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "void",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static I32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "i32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static F32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "f32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static B32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "b32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static B64_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "b64",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static BOOL_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "bool",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

pub static VEC_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&VEC_TYPE_DECL),
};
