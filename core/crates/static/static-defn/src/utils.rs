#[macro_export]
macro_rules! static_mod {
    ($name: ident = { $($items: expr),* }) => {
        pub static MODELS_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
            name: "models",
            items: &[],
            variant: EntityStaticDefnVariant::Module,
            dev_src: dev_utils::static_dev_src!(),
        };
    };
}
