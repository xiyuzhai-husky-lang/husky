pub mod connected_component;
pub mod raw_contour;
pub mod geom2d;
pub mod line_segment_sketch;
pub mod fermi;
pub mod digits;
pub mod major;

use self::connected_component::*;
use self::raw_contour::*;
use self::geom2d::*;
use self::line_segment_sketch::*;
use self::fermi::*;
use self::digits::*;
use self::major::*;

pub fn main() -> Class<MnistLabel> {
    is_one?;
    is_six?;
    is_zero?;
    is_seven?;
    is_eight?;
    is_three?;
    is_nine?;
    is_five?;
    is_two?;
    Class::Unknown
}