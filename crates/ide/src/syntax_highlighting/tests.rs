use std::time::Instant;

use expect_test::{expect_file, ExpectFile};
use ide_db::SymbolKind;
use test_utils::{bench, bench_fixture, skip_slow_tests, AssertLinear};

use crate::{fixture, FileRange, HlTag, TextRange};

/// Highlights the code given by the `ra_fixture` argument, renders the
/// result as HTML, and compares it with the HTML file given as `snapshot`.
/// Note that the `snapshot` file is overwritten by the rendered HTML.
fn check_highlighting(ra_fixture: &str, expect: ExpectFile, rainbow: bool) {
    let (analysis, file_id) = fixture::file(ra_fixture);
    let actual_html = &analysis.highlight_as_html(file_id, rainbow).unwrap();
    expect.assert_eq(actual_html)
}
