use husky_diagnostics::Diagnostic;

use crate::*;

impl HuskyComptime {
    pub fn print_diagnostics(&self) {
        let modules = self.all_modules();
        for module in modules.iter() {
            let diagnostic_reserve = self.diagnostics_reserve(*module);
            p!(module);
            p!(self.module_file(*module));
            p!(diagnostic_reserve.data());
        }
    }

    pub fn all_diagnostics(&self) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        for module in self.all_modules() {
            let diagnostics_reserve = self.diagnostics_reserve(module);
            diagnostics.extend(diagnostics_reserve.data().iter().map(|d| d.clone()));
        }
        diagnostics
    }
}
