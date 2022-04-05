use entity_route::EntityRoutePtr;

use crate::*;

pub(crate) fn collect_diagnostics(db: &dyn DiagnosticQuery, module: EntityRoutePtr) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if let Ok(table) = db.subscope_table(module) {
        diagnostics.extend(table.error_iter().map(|e| e.into()));
        diagnostics.extend(
            db.subscopes(module)
                .iter()
                .map(|subscope_id| match db.raw_entity_kind(*subscope_id) {
                    entity_route::RawEntityKind::Module => todo!(),
                    _ => collect_module_def_diagnostics(db, *subscope_id),
                })
                .flatten(),
        );
    }
    diagnostics
}

fn collect_module_def_diagnostics(
    _this: &dyn DiagnosticQuery,
    _scope: entity_route::EntityRoutePtr,
) -> Vec<Diagnostic> {
    msg_once!("todo collect module def diagnostics!");
    Vec::new()
}
