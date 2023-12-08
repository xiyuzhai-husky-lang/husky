#![allow(warnings, non_snake_case)]
use husky_core::*;

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

#[ad_hoc_task_dependency::val_item]
pub fn main() -> Class<MnistLabel> {
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