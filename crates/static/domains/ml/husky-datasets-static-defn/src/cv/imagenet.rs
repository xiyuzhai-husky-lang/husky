mod lazy_image;
mod stage0;
mod stage1;

pub use lazy_image::*;

use super::*;
use stage0::*;

pub static IMAGENET_MOD: EntityStaticDefn = EntityStaticDefn {
    name: "imagenet",
    items: &[&LAZY_IMAGE256_DEFN, &STAGE0_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};
