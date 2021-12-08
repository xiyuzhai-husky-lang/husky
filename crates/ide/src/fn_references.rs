//! This module implements a methods and free functions search in the specified file.
//! We have to skip tests, so cannot reuse file_structure module.

use hir::Semantics;
use ide_assists::utils::test_related_attribute;
use husky_lang_db::HuskyLangDatabase;
use syntax::{ast, SyntaxNode};

use crate::{SourceFileId, SourceFileRange};

pub(crate) fn find_all_methods(db: &HuskyLangDatabase, file_id: SourceFileId) -> Vec<SourceFileRange> {
    todo!()
}

fn method_range(item: SyntaxNode, file_id: SourceFileId) -> Option<SourceFileRange> {
    todo!()
}
