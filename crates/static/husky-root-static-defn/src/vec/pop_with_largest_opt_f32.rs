use super::*;

pub static VEC_POP_WITH_LARGEST_OPT_F32: EntityStaticDefn = EntityStaticDefn {
    name: "pop_with_largest_opt_f32",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::TempRefMut,
        parameters: &[StaticParameter {
            name: "f",
            modifier: ParameterModifier::None,
            ty: "(E) -> ?f32",
        }],
        output_ty: "?E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            virtual_vec_pop_with_largest_opt_f32,
            none
        )),
        output_liason: OutputModifier::Transfer,
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_pop_with_largest_opt_f32<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}
