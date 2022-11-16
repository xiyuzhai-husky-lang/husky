use crate::*;
use husky_path::PathItd;
use husky_text::{FileRanged, TextRange, TextRanged};
use husky_vm::{InstructionId, InstructionSource};
use husky_word::IdentPairDict;
use husky_xml_syntax::XmlTagKind;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlExpr {
    pub variant: XmlExprVariant,
    pub range: TextRange,
    pub file: PathItd,
    pub instruction_id: InstructionId,
}

impl TextRanged for XmlExpr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
impl FileRanged for XmlExpr {
    fn file(&self) -> PathItd {
        self.file
    }
}

impl InstructionSource for XmlExpr {
    fn instruction_id(&self) -> husky_vm::InstructionId {
        self.instruction_id
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
