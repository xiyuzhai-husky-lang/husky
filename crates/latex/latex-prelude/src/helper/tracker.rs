use sealed::*;

use crate::mode::LxMode;

#[sealed]
pub trait IsLxInput<'a>: Copy {
    fn input(self) -> &'a str;
    fn root_mode(self) -> LxMode;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxDocumentInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Root
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentBodyInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxDocumentBodyInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Rose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxFormulaInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxFormulaInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Math
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxLispInput<'a>(pub &'a str);

#[sealed]
impl<'a> IsLxInput<'a> for LxLispInput<'a> {
    fn input(self) -> &'a str {
        self.0
    }

    fn root_mode(self) -> LxMode {
        LxMode::Lisp
    }
}
