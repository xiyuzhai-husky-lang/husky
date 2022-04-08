use entity_route::EntityRoutePtr;
use file::FilePtr;

use crate::*;

pub(crate) fn collect_diagnostics(
    db: &dyn DiagnosticQuery,
    module: EntityRoutePtr,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if let Ok(table) = db.subscope_table(module) {
        diagnostics.extend(table.error_iter().map(|e| e.into()));
        for subscope_id in db.subscopes(module).iter() {
            match db.raw_entity_kind(*subscope_id) {
                entity_route::RawEntityKind::Module => todo!("check module exists"),
                _ => (),
            }
        }
    }
    let file = db.module_file(module).unwrap();
    collect_ast_errors(db, file, &mut diagnostics);
    diagnostics
}

fn collect_ast_errors(db: &dyn DiagnosticQuery, file: FilePtr, diagnostics: &mut Vec<Diagnostic>) {
    let ast_text = db.ast_text(file).unwrap();
    for node in ast_text.folded_results.nodes() {
        match node.value {
            Ok(_) => (),
            Err(ref error) => diagnostics.push(error.into()),
        }
    }
}

fn collect_infer_ty_errors() {}

fn collect_infer_contract_errors() {}
