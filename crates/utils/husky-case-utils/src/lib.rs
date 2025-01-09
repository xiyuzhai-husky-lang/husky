#[cfg(feature = "relative_path")]
mod relative_path;

pub use convert_case::Case;

use convert_case::Casing;

pub trait ToCase {
    type Output;

    fn to_case(self, case: Case) -> Self::Output;
}
