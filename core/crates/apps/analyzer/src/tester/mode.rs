use print_utils::p;

use super::flags::HuskyAnalyzerTester;

pub enum AnalyzerTesterMode {
    TestDiagnostics,
    TestFoldingRanges,
    TestSemanticTokens,
    TestQualifiedTys,
}

impl AnalyzerTesterMode {
    pub fn from_flags(flags: &HuskyAnalyzerTester) -> Self {
        match flags.mode.as_str() {
            "test-diagnostics" => AnalyzerTesterMode::TestDiagnostics,
            "test-folding-ranges" => AnalyzerTesterMode::TestFoldingRanges,
            "test-semantic-tokens" => AnalyzerTesterMode::TestSemanticTokens,
            "test-qualified-tys" => AnalyzerTesterMode::TestQualifiedTys,
            _ => {
                p!(flags.mode);
                todo!()
            }
        }
    }
}
