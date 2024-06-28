use super::*;
use husky_code_lens::lsp_support::module_lsp_code_lenses_unresolved;

pub(crate) fn handle_code_lens(
    snapshot: AnalyzerDBSnapshot,
    params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    let _p = husky_profile_utils::span("handle_code_lens");
    let path = from_proto::path_from_url(&params.text_document.uri)?;
    let module_path = snapshot.resolve_module_path_and_update_live_packages(&path)?;
    let code_lens = module_lsp_code_lenses_unresolved(module_path, &snapshot);
    Ok(Some(code_lens))
}

pub(crate) fn handle_code_lens_resolve(
    _snapshot: AnalyzerDBSnapshot,
    _code_lens: CodeLens,
) -> Result<CodeLens> {
    msg_once!("todo handle code lens resolve!");
    todo!()
}
