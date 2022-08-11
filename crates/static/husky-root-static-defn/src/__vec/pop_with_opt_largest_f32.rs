use super::*;

pub static VEC_POP_WITH_OPT_LARGEST_F32: EntityStaticDefn = EntityStaticDefn {
    name: "pop_with_opt_largest_f32",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::TempRefMut,
        parameters: &[StaticParameter {
            name: "f",
            liason: ParameterLiason::Pure,
            ty: "(E) -> ?f32",
        }],
        output_ty: "?E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_pop, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};
