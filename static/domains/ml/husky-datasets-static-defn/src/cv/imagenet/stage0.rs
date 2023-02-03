use super::*;
use husky_static_defn::static_mod;

static_mod!(stage0 = { new_husky_dataset });
pub static NEW_HUSKY_DATASET_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "new_husky_dataset",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicParameterDecl::None,
        return_ty: "Dataset<cv::datasets::imagenet::LazyImage256, cv::datasets::imagenet::stage0::HuskyOrOther>",
        output_liason: OutputModifier::Transfer,
        linkage: transfer_linkage!(
            |_, _| __Register::new_box(new_husky_dataset, &__DATASET_VTABLE),
            some base new_husky_dataset as fn () -> Dataset<'static>
        )
        .into(),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub fn new_husky_dataset<'eval>() -> Dataset<'eval> {
    // Dataset::new(MnistDataset::new(35016232u64))
    todo!()
}
