use sealed::*;

use crate::mode::LxMode;

#[sealed]
pub trait IsLxInput<'a>: Copy {
    fn input(self) -> &'a str;
    fn root_mode(self) -> LxMode;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentTrackerInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxDocumentTrackerInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Root
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentBodyTrackerInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxDocumentBodyTrackerInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Rose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxFormulaTrackerInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxFormulaTrackerInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Math
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxLispTrackerInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxLispTrackerInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Lisp
    }
}
