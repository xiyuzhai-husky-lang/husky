use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;
use vm::{InstructionId, InstructionSource};
use word::{IdentDict, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlExpr {
    pub variant: XmlExprVariant,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlExprVariant {
    Value(Arc<EagerExpr>),
    Tag {
        tag_kind: XmlTagKind,
        props: IdentPairDict<Arc<EagerExpr>>,
    },
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTagKind {
    Point2d,
    Contour,
    Arrow2d,
}

impl XmlTagKind {
    pub fn as_str(self) -> &'static str {
        match self {
            XmlTagKind::Point2d => "Point2d",
            XmlTagKind::Arrow2d => "Arrow2d",
            XmlTagKind::Contour => "Contour",
        }
    }

    pub fn from_ident(ident: CustomIdentifier) -> Self {
        match ident.as_str() {
            "Point2d" => XmlTagKind::Point2d,
            "Contour" => XmlTagKind::Contour,
            "Arrow2d" => XmlTagKind::Arrow2d,
            _ => todo!(),
        }
    }
}
