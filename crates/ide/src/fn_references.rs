//! This module implements a methods and free functions search in the specified file.
//! We have to skip tests, so cannot reuse file_structure module.

use hir::Semantics;
use ide_assists::utils::test_related_attribute;
use ide_db::IdeDatabase;
use syntax::{ast, SyntaxNode};

use crate::{FileID, FileRange};

pub(crate) fn find_all_methods(db: &IdeDatabase, file_id: FileID) -> Vec<FileRange> {
    todo!()
}

fn method_range(item: SyntaxNode, file_id: FileID) -> Option<FileRange> {
    todo!()
}
