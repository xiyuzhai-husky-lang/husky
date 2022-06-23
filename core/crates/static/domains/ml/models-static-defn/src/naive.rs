use super::*;
use dev_utils::static_dev_src;
use static_defn::*;

static_mod! { naive = { naive_i32 } }

pub static NAIVE_I32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "naive_i32",
    items: &[],
    variant: EntityStaticDefnVariant::Morphism {
        generic_parameters: &[],
        parameters: &[StaticParameter {
            name: "a",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        morphism_variant: StaticMorphismVariant::Model {
            train: (),
            eval: (),
        },
    },
    dev_src: static_dev_src!(),
};
