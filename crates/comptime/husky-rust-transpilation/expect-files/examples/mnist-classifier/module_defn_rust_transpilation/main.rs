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

pub fn main() -> Class<MnistLabel> {
    is_one()?;
    is_six()?;
    is_zero()?;
    is_seven()?;
    is_eight()?;
    is_three()?;
    is_nine()?;
    is_five()?;
    is_two()?;
    Class::Unknown
}