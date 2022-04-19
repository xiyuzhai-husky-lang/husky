use vm::{InputContract, OutputContract};

use crate::*;

pub static STD_OPS_MODULE_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[
        ("Index", &INDEX_TRAIT_DEFN),
        ("IndexMut", &INDEX_TRAIT_DEFN),
    ],
    decl: StaticEntityDecl::Module,
};

pub static INDEX_TRAIT_DEFN: StaticEntityDefn = StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Trait(&INDEX_TRAIT_DECL),
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
        StaticTraitMemberDecl::Method(StaticMethodDecl {
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
