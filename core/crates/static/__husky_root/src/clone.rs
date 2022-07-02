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
                this_liason: ParameterLiason::Pure,
                parameters: &[],
                output_ty: "This",
                spatial_parameters: &[],
                method_static_defn_kind: MethodStaticDefnKind::TraitMethod,
                opt_linkage: Some(specific_transfer_linkage!(
                    |values| Ok(values[0].clone_into_stack()),
                    1
                )),
                output_liason: OutputLiason::Transfer,
            },
            dev_src: __static_dev_src!(),
        }],
        spatial_parameters: &[],
    },
    dev_src: __static_dev_src!(),
};
