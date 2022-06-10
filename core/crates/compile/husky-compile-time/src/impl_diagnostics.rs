use crate::*;

impl HuskyCompileTime {
    pub fn print_diagnostics(&self) {
        let modules = self.all_modules();
        for module in modules.iter() {
            let diagnostic_reserve = self.diagnostics_reserve(*module);
            p!(module);
            p!(self.module_file(*module));
            p!(diagnostic_reserve.data());
        }
    }
}
