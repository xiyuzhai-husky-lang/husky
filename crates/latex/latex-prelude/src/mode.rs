use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxMode {
    Lisp,
    Math,
    Root,
    Rose,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxModeSet(NonZeroU64);

impl LxModeSet {
    pub const LISP: Self = Self(unsafe { NonZeroU64::new_unchecked(1 << 1 + 1) });
    pub const MATH: Self = Self(unsafe { NonZeroU64::new_unchecked(1 << 2 + 1) });
    pub const ROOT: Self = Self(unsafe { NonZeroU64::new_unchecked(1 << 3 + 1) });
    pub const ROSE: Self = Self(unsafe { NonZeroU64::new_unchecked(1 << 4 + 1) });
    pub const NAME: Self = Self(unsafe { NonZeroU64::new_unchecked(1 << 5 + 1) });
}

impl LxModeSet {
    pub fn allowed_in_lisp(self) -> bool {
        self.0.get() & Self::LISP.0.get() != 0
    }

    pub fn allowed_in_math(self) -> bool {
        self.0.get() & Self::MATH.0.get() != 0
    }

    pub fn allowed_in_root(self) -> bool {
        self.0.get() & Self::ROOT.0.get() != 0
    }

    pub fn allowed_in_rose(self) -> bool {
        self.0.get() & Self::ROSE.0.get() != 0
    }

    pub fn allowed_in_name(self) -> bool {
        self.0.get() & Self::NAME.0.get() != 0
    }
}

impl std::ops::BitOr for LxModeSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl From<&[LxMode]> for LxModeSet {
    fn from(modes: &[LxMode]) -> Self {
        modes
            .iter()
            .copied()
            .fold(Self::LISP, |acc, mode| acc | mode.into())
    }
}

impl From<LxMode> for LxModeSet {
    fn from(mode: LxMode) -> Self {
        match mode {
            LxMode::Lisp => Self::LISP,
            LxMode::Math => Self::MATH,
            LxMode::Root => Self::ROOT,
            LxMode::Rose => Self::ROSE,
            LxMode::Name => Self::NAME,
        }
    }
}
