mod flags;
mod mode;
mod test_diagnostics;
mod test_folding_ranges;
mod test_qualified_tys;
mod test_semantic_tokens;

pub use mode::*;

use print_utils::p;
use test_diagnostics::test_diagnostics;
use test_folding_ranges::test_folding_ranges;
use test_qualified_tys::test_qualified_tys;
use test_semantic_tokens::test_semantic_tokens;
use test_utils::test_all_packages_in_dir;

pub fn test_all() {
    let flags = match flags::HuskyAnalyzerTester::from_env() {
        Ok(flags) => flags,
        Err(e) => {
            p!(e);
            panic!()
        }
    };
    let mode = AnalyzerTesterMode::from_flags(&flags);
    let dir = flags.dir;
    test_all_packages_in_dir(
        dir,
        match mode {
            AnalyzerTesterMode::TestDiagnostics => test_diagnostics,
            AnalyzerTesterMode::TestFoldingRanges => test_folding_ranges,
            AnalyzerTesterMode::TestSemanticTokens => test_semantic_tokens,
            AnalyzerTesterMode::TestQualifiedTys => test_qualified_tys,
        },
    )
}
