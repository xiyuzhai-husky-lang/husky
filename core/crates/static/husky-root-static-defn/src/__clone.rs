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
                return_ty: "This",
                spatial_parameters: &[],
                method_static_defn_kind: MethodStaticDefnKind::TraitMethod,
                opt_linkage: Some(transfer_linkage!(
                    |_, values| unsafe { values[0].intrinsic_clone() },
                    none
                )),
                output_liason: OutputLiason::Transfer,
            },
            dev_src: static_dev_src!(),
        }],
        spatial_parameters: &[],
    },
    dev_src: static_dev_src!(),
};
