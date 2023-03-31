use crate::*;
use husky_expr::ExprRegion;

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

    pub(crate) fn visit_atom(
        &mut self,
        atom: &impl Diagnose<Context<'a> = SheetDiagnosticsContext<'a>>,
    ) {
        self.diagnostics.push(atom.to_diagnostic(&self.context))
    }
    pub(crate) fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub(crate) fn db(&self) -> &'a dyn DiagnosticsDb {
        self.context.db()
    }

    pub(crate) fn ctx(&self) -> &SheetDiagnosticsContext<'a> {
        &self.context
    }
}

pub(crate) struct RegionDiagnosticsCollector<'a, 'b> {
    // todo: a little redundant
    // sheet_collector contains same fields as in context
    context: RegionDiagnosticsContext<'a>,
    sheet_collector: &'b mut SheetDiagnosticsCollector<'a>,
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    pub(crate) fn new(
        db: &'a dyn DiagnosticsDb,
        expr_region: ExprRegion,
        sheet_collector: &'b mut SheetDiagnosticsCollector<'a>,
    ) -> Self {
        Self {
            context: RegionDiagnosticsContext::new(db, expr_region),
            sheet_collector,
        }
    }

    pub(crate) fn visit_atom(
        &mut self,
        atom: &impl Diagnose<Context<'a> = RegionDiagnosticsContext<'a>>,
    ) {
        self.sheet_collector
            .diagnostics
            .push(atom.to_diagnostic(&self.context))
    }

    pub(crate) fn db(&self) -> &'a dyn DiagnosticsDb {
        self.context.db()
    }
}
