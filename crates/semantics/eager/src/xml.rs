use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;
use vm::{InstructionId, InstructionSource};
use word::{IdentDict, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlExpr {
    pub kind: XmlExprKind,
    pub props: IdentPairDict<Arc<EagerExpr>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub instruction_id: InstructionId,
}

impl InstructionSource for XmlExpr {
    fn instruction_id(&self) -> vm::InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlExprKind {
    Point2d,
    Contour,
    Arrow2d,
}

impl XmlExprKind {
    pub fn as_str(self) -> &'static str {
        match self {
            XmlExprKind::Point2d => "Point2d",
            XmlExprKind::Arrow2d => "Arrow2d",
            XmlExprKind::Contour => "Contour",
        }
    }

    pub fn from_ident(ident: CustomIdentifier) -> Self {
        match ident.as_str() {
            "Point2d" => XmlExprKind::Point2d,
            "Contour" => XmlExprKind::Contour,
            "Arrow2d" => XmlExprKind::Arrow2d,
            _ => todo!(),
        }
    }
}
