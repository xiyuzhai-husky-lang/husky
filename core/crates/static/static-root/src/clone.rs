use crate::*;

pub static CLONE_TRAIT_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Clone",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "Clone",
        members: &[EntityStaticDefn {
            name: "clone",
            subscopes: &[],
            variant: EntityStaticDefnVariant::Method {
                this_liason: ParameterLiason::Pure,
                parameters: &[],
                output_ty: "This",
                generic_parameters: &[],
                kind: MethodStaticDefnVariant::TraitMethod {
                    opt_default_source: Some(LinkageSource::Transfer(linkage!(
                        |values| Ok(values[0].clone_into_stack()),
                        1
                    ))),
                },
                output_liason: OutputLiason::Transfer,
            },
            dev_src: static_dev_src!(),
        }],
        generic_parameters: &[],
    },
    dev_src: static_dev_src!(),
};
