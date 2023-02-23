use crate::*;
use husky_expr::ExprRegion;

#[derive(Default)]
pub(crate) struct RegionDiagnosticsCollectorCenter {
    diagnostics: Vec<Diagnostic>,
}

impl RegionDiagnosticsCollectorCenter {
    pub(crate) fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

pub(crate) struct RegionDiagnosticsCollector<'a> {
    context: RegionDiagnosticsContext<'a>,
    center: &'a mut RegionDiagnosticsCollectorCenter,
}

impl<'a> RegionDiagnosticsCollector<'a> {
    pub(crate) fn new(
        db: &'a dyn DiagnosticsDb,
        expr_region: ExprRegion,
        center: &'a mut RegionDiagnosticsCollectorCenter,
    ) -> Self {
        Self {
            context: RegionDiagnosticsContext::new(db, expr_region),
            center,
        }
    }

    pub(crate) fn visit_atom(
        &mut self,
        atom: &impl Diagnose<Context<'a> = RegionDiagnosticsContext<'a>>,
    ) {
        self.center
            .diagnostics
            .push(atom.to_diagnostic(&self.context))
    }
}

pub(crate) struct SheetDiagnosticsCollector<'a> {
    context: SheetDiagnosticsContext<'a>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> SheetDiagnosticsCollector<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, module_path: ModulePath) -> Self {
        Self {
            diagnostics: vec![],
            context: SheetDiagnosticsContext::new(db, module_path),
        }
    }

    pub(crate) fn collect(&mut self, e: &impl Diagnose<Context<'a> = SheetDiagnosticsContext<'a>>) {
        self.diagnostics.push(e.to_diagnostic(&self.context))
    }
}
