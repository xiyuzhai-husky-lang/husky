use vm::{InputContract, OutputContract};

use crate::*;

pub static STD_OPS_MODULE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "ops",
    subscopes: &[
        ("Index", &INDEX_TRAIT_DEFN),
        ("IndexMut", &INDEX_TRAIT_DEFN),
    ],
    variant: StaticEntityDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub static INDEX_TRAIT_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "Index",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Trait(&INDEX_TRAIT_DECL),
    dev_src: dev_utils::static_dev_src!(),
};

pub static INDEX_TRAIT_DECL: StaticTraitDecl = StaticTraitDecl {
    base_route: "std::ops::Index",
    generic_placeholders: &[StaticGenericPlaceholder {
        name: "Idx",
        variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
    }],
    members: &[
        StaticTraitMemberDecl::Type {
            name: "Output",
            traits: &[],
        },
        StaticTraitMemberDecl::Method(StaticMethodDefn {
            name: "index",
            this_contract: InputContract::MemberAccess,
            inputs: &[],
            output_ty: "This::Output",
            output_contract: OutputContract::MemberAccess,
            generic_placeholders: &[],
            kind: StaticMethodKind::TraitMethod("std::ops::Index"),
        }),
    ],
};
