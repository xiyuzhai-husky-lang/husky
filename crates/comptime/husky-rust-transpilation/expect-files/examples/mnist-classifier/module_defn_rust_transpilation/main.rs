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
    unveil!(malamute::Class<mnist::MnistLabel>, is_one(), (mnist::MnistLabel::One, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_six(), (mnist::MnistLabel::Six, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_zero(), (mnist::MnistLabel::Zero, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_seven(), (mnist::MnistLabel::Seven, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_eight(), (mnist::MnistLabel::Eight, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_three(), (mnist::MnistLabel::Three, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_nine(), (mnist::MnistLabel::Nine, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_five(), (mnist::MnistLabel::Five, ));
    unveil!(malamute::Class<mnist::MnistLabel>, is_two(), (mnist::MnistLabel::Two, ));
    Class::Unknown
}