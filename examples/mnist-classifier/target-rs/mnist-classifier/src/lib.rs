#![allow(warnings, non_snake_case)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};

ad_hoc_task_dependency::init_crate!();

pub mod connected_component;
pub mod raw_contour;
pub mod geom2d;
pub mod line_segment_sketch;
pub mod fermi;
pub mod digits;
pub mod major;

pub use self::connected_component::*;
pub use self::raw_contour::*;
pub use self::geom2d::*;
pub use self::line_segment_sketch::*;
pub use self::fermi::*;
pub use self::digits::*;
pub use self::major::*;

use malamute::*;
use mnist::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 0)]
pub fn main() -> malamute::Class<mnist::MnistLabel> {
    unveil!(is_one());
    unveil!(is_six());
    unveil!(is_zero());
    unveil!(is_seven());
    unveil!(is_eight());
    unveil!(is_three());
    unveil!(is_nine());
    unveil!(is_five());
    unveil!(is_two());
    Class::Unknown
}