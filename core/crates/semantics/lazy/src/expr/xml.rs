use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;
use vm::{InstructionId, InstructionSource, XmlTagKind};
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
    Value(Arc<LazyExpr>),
    Tag {
        tag_kind: XmlTagKind,
        props: IdentPairDict<Arc<LazyExpr>>,
    },
}
