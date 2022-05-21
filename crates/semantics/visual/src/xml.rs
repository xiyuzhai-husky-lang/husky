use std::sync::Arc;

use file::FilePtr;
use semantics_eager::EagerExpr;
use text::TextRange;
use word::{IdentDict, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlExpr {
    kind: XmlExprKind,
    props: IdentPairDict<Arc<EagerExpr>>,
    range: TextRange,
    file: FilePtr,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlExprKind {
    Point2d,
    Contour2d,
    Arrow2d,
}
