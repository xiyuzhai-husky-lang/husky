use ast::AstText;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use print_utils::p;
use semantics_error::SemanticErrorVariant;

use crate::*;

pub(crate) fn collect_diagnostics(
    db: &dyn DiagnosticQuery,
    module: EntityRoutePtr,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if let Ok(table) = db.subroute_table(module) {
        diagnostics.extend(table.error_iter().map(|e| e.into()));
        for subscope_id in db.subscopes(module).iter() {
            match db.raw_entity_kind(*subscope_id) {
                entity_route::EntityKind::Module => todo!("check module exists"),
                _ => (),
            }
        }
    }
    let file = db.module_file(module).unwrap();
    collect_lex_errors(db, file, &mut diagnostics);
    collect_ast_errors(db, file, &mut diagnostics);
    collect_infer_ty_errors(db, file, &mut diagnostics);
    collect_infer_contract_errors(db, file, &mut diagnostics);
    // collect_semantic_errors(db, file, &mut diagnostics);
    diagnostics
}

fn collect_lex_errors(db: &dyn DiagnosticQuery, file: FilePtr, diagnostics: &mut Vec<Diagnostic>) {
    let tokenized_text = db.tokenized_text(file).unwrap();
    diagnostics.extend(tokenized_text.errors.iter().map(|error| error.into()))
}

fn collect_ast_errors(db: &dyn DiagnosticQuery, file: FilePtr, diagnostics: &mut Vec<Diagnostic>) {
    let ast_text = db.ast_text(file).unwrap();
    for node in ast_text.folded_results.nodes() {
        match node.value {
            Ok(_) => (),
            Err(ref error) => match error.variant {
                AstErrorVariant::Original { .. } => diagnostics.push(error.into()),
                AstErrorVariant::Derived => (),
            },
        }
    }
}

fn collect_infer_ty_errors(
    db: &dyn DiagnosticQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let ty_sheet = db.entity_route_sheet(file).unwrap();
    for error in ty_sheet.original_errors() {
        diagnostics.push(error.into());
    }
}

fn collect_infer_contract_errors(
    db: &dyn DiagnosticQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let contract_sheet = db.contract_sheet(file).unwrap();
    for error in contract_sheet.errors() {
        diagnostics.push(error.into());
    }
}

fn collect_semantic_errors(
    db: &dyn DiagnosticQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let module = db.module(file).unwrap();
    match db.entity_defn(module) {
        Ok(_) => (),
        Err(e) => match e.variant {
            SemanticErrorVariant::Derived { .. } => (),
            SemanticErrorVariant::Original { .. } => diagnostics.push(e.into()),
        },
    }
}
