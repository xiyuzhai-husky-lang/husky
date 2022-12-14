use husky_completion::HuskyCompletionQuery;
use husky_diagnostics::DiagnosticsDb;
use husky_hover::HoverDb;

use crate::*;

impl HoverDb for AnalysisHost {
    fn hover_config(&self) -> &husky_hover::HoverConfig {
        todo!()
    }
}

impl HuskyCompletionQuery for AnalysisHost {}
