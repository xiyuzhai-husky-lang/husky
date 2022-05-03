use print_utils::p;

use super::flags::HuskyAnalyzerTester;

pub enum AnalyzerTesterMode {
    TestDiagnostics,
    TestFoldingRanges,
    TestSemanticTokens,
}

impl AnalyzerTesterMode {
    pub fn from_flags(flags: &HuskyAnalyzerTester) -> Self {
        match flags.mode.as_str() {
            "test-diagnostics" => AnalyzerTesterMode::TestDiagnostics,
            "test-folding-ranges" => AnalyzerTesterMode::TestFoldingRanges,
            "test-semantic-tokens" => AnalyzerTesterMode::TestSemanticTokens,
            _ => {
                p!(flags.mode);
                todo!()
            }
        }
    }
}
