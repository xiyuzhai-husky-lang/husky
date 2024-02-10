use ecow::EcoString;

use crate::foundations::{func, scope, ty, Repr};
use crate::layout::{Axis, Side};

/// The four directions into which content can be laid out.
///
///  Possible values are:
/// - `{ltr}`: Left to right.
/// - `{rtl}`: Right to left.
/// - `{ttb}`: Top to bottom.
/// - `{btt}`: Bottom to top.
///
/// These values are available globally and
/// also in the direction type's scope, so you can write either of the following
/// two:
/// ```example
/// #stack(dir: rtl)[A][B][C]
/// #stack(dir: direction.rtl)[A][B][C]
/// ```
#[ty(scope, name = "direction")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TypstLayoutDirection {
    /// Left to right.
    LeftRight,
    /// Right to left.
    RightLeft,
    /// Top to bottom.
    TopDown,
    /// Bottom to top.
    BottomUp,
}

impl TypstLayoutDirection {
    /// Whether this direction points into the positive coordinate direction.
    ///
    /// The positive directions are left-to-right and top-to-bottom.
    pub const fn is_positive(self) -> bool {
        match self {
            Self::LeftRight | Self::TopDown => true,
            Self::RightLeft | Self::BottomUp => false,
        }
    }
}

#[scope]
impl TypstLayoutDirection {
    pub const LEFT_RIGHT: Self = Self::LeftRight;
    pub const RIGHT_LEFT: Self = Self::RightLeft;
    pub const TOP_DOWN: Self = Self::TopDown;
    pub const BottomUp: Self = Self::BottomUp;

    /// The axis this direction belongs to, either `{"horizontal"}` or
    /// `{"vertical"}`.
    ///
    /// ```example
    /// #ltr.axis() \
    /// #ttb.axis()
    /// ```
    #[func]
    pub const fn axis(self) -> Axis {
        match self {
            Self::LeftRight | Self::RightLeft => Axis::X,
            Self::TopDown | Self::BottomUp => Axis::Y,
        }
    }

    /// The start point of this direction, as an alignment.
    ///
    /// ```example
    /// #ltr.start() \
    /// #rtl.start() \
    /// #ttb.start() \
    /// #btt.start()
    /// ```
    #[func]
    pub const fn start(self) -> Side {
        match self {
            Self::LeftRight => Side::Left,
            Self::RightLeft => Side::Right,
            Self::TopDown => Side::Top,
            Self::BottomUp => Side::Bottom,
        }
    }

    /// The end point of this direction, as an alignment.
    ///
    /// ```example
    /// #ltr.end() \
    /// #rtl.end() \
    /// #ttb.end() \
    /// #btt.end()
    /// ```
    #[func]
    pub const fn end(self) -> Side {
        match self {
            Self::LeftRight => Side::Right,
            Self::RightLeft => Side::Left,
            Self::TopDown => Side::Bottom,
            Self::BottomUp => Side::Top,
        }
    }

    /// The inverse direction.
    ///
    /// ```example
    /// #ltr.inv() \
    /// #rtl.inv() \
    /// #ttb.inv() \
    /// #btt.inv()
    /// ```
    #[func(title = "Inverse")]
    pub const fn inv(self) -> TypstLayoutDirection {
        match self {
            Self::LeftRight => Self::RightLeft,
            Self::RightLeft => Self::LeftRight,
            Self::TopDown => Self::BottomUp,
            Self::BottomUp => Self::TopDown,
        }
    }
}

impl Repr for TypstLayoutDirection {
    fn repr(&self) -> EcoString {
        match self {
            Self::LeftRight => "ltr".into(),
            Self::RightLeft => "rtl".into(),
            Self::TopDown => "ttb".into(),
            Self::BottomUp => "btt".into(),
        }
    }
}
