pub use paste::paste;

#[macro_export]
macro_rules! static_mod {
    ($name: ident = {$($items: expr),*}) => {
        static_defn::utils::paste! {
            pub static [<$name:upper _DEFN>]: &EntityStaticDefn = &EntityStaticDefn {
                name: stringify!($name),
                items: &[$(&[<$items:upper _DEFN>]),*],
                variant: EntityStaticDefnVariant::Module,
                dev_src: husky_dev_utils::static_dev_src!(),
            };
        }
    };
}
