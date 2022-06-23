use dev_utils::static_dev_src;

use crate::*;

pub static STD_OPS_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ops",
    items: &[&STD_OPS_INDEX_DEFN, &STD_OPS_INDEX_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_OPS_INDEX_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Index",
    items: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "std::ops::Index",
        generic_parameters: &[StaticGenericPlaceholder {
            name: "Idx",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        members: &[
            EntityStaticDefn {
                name: "Output",
                items: &[],
                variant: EntityStaticDefnVariant::TraitAssociatedType {
                    trai: "std::ops::Index",
                    traits: &[],
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "index",
                items: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_liason: ParameterLiason::MemberAccess,
                    parameters: &[],
                    output_ty: "This::Output",
                    output_liason: OutputLiason::MemberAccess {
                        member_liason: MemberLiason::Mutable,
                    },
                    generic_parameters: &[],
                    kind: MethodStaticDefnVariant::TraitMethod {
                        opt_default_source: None,
                    },
                },
                dev_src: static_dev_src!(),
            },
        ],
    },
    dev_src: dev_utils::static_dev_src!(),
};
