//! Utilities for creating `Analysis` instances for tests.
use hir::db::DefDatabase;
use ide_db::base_db::fixture::ChangeFixture;
use test_utils::{extract_annotations, RangeOrOffset};

use crate::{Analysis, AnalysisHost, FileID, FilePosition, FileRange};

/// Creates analysis for a single file.
pub(crate) fn file(ra_fixture: &str) -> (Analysis, FileID) {
    todo!()
}

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub(crate) fn position(ra_fixture: &str) -> (Analysis, FilePosition) {
    todo!()
}

/// Creates analysis for a single file, returns range marked with a pair of $0.
pub(crate) fn range(ra_fixture: &str) -> (Analysis, FileRange) {
    todo!()
}

/// Creates analysis for a single file, returns range marked with a pair of $0 or a position marked with $0.
pub(crate) fn range_or_position(ra_fixture: &str) -> (Analysis, FileID, RangeOrOffset) {
    todo!()
}

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub(crate) fn annotations(ra_fixture: &str) -> (Analysis, FilePosition, Vec<(FileRange, String)>) {
    todo!()
}

/// Creates analysis from a multi-file fixture with annonations without $0
pub(crate) fn annotations_without_marker(ra_fixture: &str) -> (Analysis, Vec<(FileRange, String)>) {
    todo!()
}
