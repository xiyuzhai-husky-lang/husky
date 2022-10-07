use husky_entity_route::EntityRoutePtr;
use husky_file::FilePtr;

use crate::*;

pub(crate) fn collect_module_diagnostics(
    db: &dyn DiagnosticSalsaQuery,
    module: EntityRoutePtr,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let file = match db.module_file(module) {
        Ok(file) => file,
        Err(e) => return vec![e.into()],
    };
    collect_module_entity_syntax_errors(db, module, &mut diagnostics);
    collect_module_lex_errors(db, file, &mut diagnostics);
    collect_module_ast_errors(db, file, &mut diagnostics);
    collect_module_infer_ty_errors(db, file, &mut diagnostics);
    collect_module_infer_contract_errors(db, file, &mut diagnostics);
    collect_module_infer_qualified_ty_errors(db, file, &mut diagnostics);
    diagnostics
}

fn collect_module_entity_syntax_errors(
    db: &dyn DiagnosticSalsaQuery,
    module: EntityRoutePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    collect_entity_syntax_errors(db, module, diagnostics)
}

fn collect_entity_syntax_errors(
    db: &dyn DiagnosticSalsaQuery,
    parent: EntityRoutePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let table = db.subroute_table(parent).unwrap();
    diagnostics.extend(table.error_iter().map(|e| e.into()));
    for subroute in table.non_module_subroute_iter(db.upcast(), parent) {
        collect_entity_syntax_errors(db, subroute, diagnostics)
    }
}

fn collect_module_lex_errors(
    db: &dyn DiagnosticSalsaQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let tokenized_text = db.tokenized_text(file).unwrap();
    diagnostics.extend(tokenized_text.errors.iter().map(|error| error.into()))
}

fn collect_module_ast_errors(
    db: &dyn DiagnosticSalsaQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
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

fn collect_module_infer_ty_errors(
    db: &dyn DiagnosticSalsaQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let ty_sheet = db.entity_route_sheet(file).unwrap();
    for error in ty_sheet.original_errors() {
        diagnostics.push(error.into());
    }
}

fn collect_module_infer_contract_errors(
    db: &dyn DiagnosticSalsaQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let contract_sheet = db.contract_sheet(file).unwrap();
    for error in contract_sheet.errors() {
        diagnostics.push(error.into());
    }
}

fn collect_module_infer_qualified_ty_errors(
    db: &dyn DiagnosticSalsaQuery,
    file: FilePtr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let qualified_ty_sheet = db.qualified_ty_sheet(file).unwrap();
    for error in qualified_ty_sheet.original_errors() {
        diagnostics.push(error.into());
    }
}
