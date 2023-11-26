use crate::*;

use husky_syn_expr::SynExprRegion;

pub(crate) struct ModuleDiagnosticsCollector<'a> {
    context: SheetDiagnosticsContext<'a>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> ModuleDiagnosticsCollector<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, module_path: ModulePath) -> Self {
        Self {
            diagnostics: vec![],
            context: SheetDiagnosticsContext::new(db, module_path),
        }
    }

    pub(crate) fn region_collector<'b>(
        &'b mut self,
        syn_expr_region: SynExprRegion,
    ) -> RegionDiagnosticsCollector<'a, 'b> {
        let db = self.context.db();
        RegionDiagnosticsCollector::new(db, syn_expr_region, self)
    }

    pub(crate) fn visit_atom(
        &mut self,
        atom: &impl Diagnose<Context<'a> = SheetDiagnosticsContext<'a>>,
    ) {
        self.diagnostics.push(atom.to_diagnostic(&self.context));
    }
    pub(crate) fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
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
    sheet_collector: &'b mut ModuleDiagnosticsCollector<'a>,
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        syn_expr_region: SynExprRegion,
        sheet_collector: &'b mut ModuleDiagnosticsCollector<'a>,
    ) -> Self {
        Self {
            context: RegionDiagnosticsContext::new(db, syn_expr_region),
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

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.context.db()
    }
}
