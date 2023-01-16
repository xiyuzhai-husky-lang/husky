use super::*;

pub(crate) fn handle_semantic_tokens_full(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    use std::sync::atomic::AtomicU32;
    static SEMANTIC_TOKENS_RESULT_ID_NEXT: AtomicU32 = AtomicU32::new(1);

    let path = from_lsp_types::path_from_url(&params.text_document.uri)?;
    let module_path = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    let semantic_tokens_ext = snapshot.semantic_tokens_ext(module_path)?;
    // Unconditionally cache the tokens
    snapshot.cache_semantic_tokens_ext(params.text_document.uri, semantic_tokens_ext);
    Ok(Some(SemanticTokensResult::Tokens(
        lsp_types::SemanticTokens {
            result_id: Some(
                SEMANTIC_TOKENS_RESULT_ID_NEXT
                    .fetch_add(1, Ordering::SeqCst)
                    .to_string(),
            ),
            data: semantic_tokens_ext.to_vec(),
        },
    )))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    // ad hoc
    todo!()
}

pub(crate) fn handle_semantic_tokens_range(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    // ad hoc
    let path = from_lsp_types::path_from_url(&params.text_document.uri)?;
    let module_path = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    let semantic_tokens_ext = snapshot.semantic_tokens_ext(module_path)?;
    Ok(Some(SemanticTokensRangeResult::Tokens(
        lsp_types::SemanticTokens {
            result_id: None,
            data: semantic_tokens_ext.to_vec(),
        },
    )))
}
