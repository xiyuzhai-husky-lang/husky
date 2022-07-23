use husky_print_utils::p;

use super::flags::HuskyAnalyzerPrinter;

pub enum AnalyzerPrinterMode {
    PrintDiagnostics,
    PrintFoldingRanges,
    PrintSemanticTokens,
    PrintQualifiedTys,
}

impl AnalyzerPrinterMode {
    pub fn from_flags(flags: &HuskyAnalyzerPrinter) -> Self {
        match flags.mode.as_str() {
            "print-diagnostics" => AnalyzerPrinterMode::PrintDiagnostics,
            "print-folding-ranges" => AnalyzerPrinterMode::PrintFoldingRanges,
            "print-semantic-tokens" => AnalyzerPrinterMode::PrintSemanticTokens,
            "print-qualified-tys" => AnalyzerPrinterMode::PrintQualifiedTys,
            _ => {
                p!(flags.mode);
                todo!()
            }
        }
    }
}
