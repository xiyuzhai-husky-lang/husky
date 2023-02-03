use crate::*;

pub static CLONE_TRAIT_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Clone",
    items: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "Clone",
        members: &[EntityStaticDefn {
            name: "clone",
            items: &[],
            variant: EntityStaticDefnVariant::Method {
                this_modifier: ParameterModifier::None,
                parameters: &[],
                return_ty: "This",
                spatial_parameters: &[],
                method_static_defn_kind: MethodStaticDefnKind::TraitMethod,
                opt_linkage: Some(transfer_linkage!(
                    |values, _| unsafe { values[0].intrinsic_clone() },
                    none
                )),
                output_liason: OutputModifier::Transfer,
            },
            dev_src: static_dev_src!(),
        }],
        spatial_parameters: &[],
    },
    dev_src: static_dev_src!(),
};
