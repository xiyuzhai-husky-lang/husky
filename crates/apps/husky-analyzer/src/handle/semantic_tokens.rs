use dashmap::mapref::entry::Entry;
use lsp_types::{SemanticToken, SemanticTokens, SemanticTokensEdit, Url};

use super::*;
use std::{sync::atomic::AtomicU32, time::Instant};

pub(crate) fn handle_semantic_tokens_full(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    eprintln!("start handle_semantic_tokens_full");
    let _p = husky_profile_utils::span("handle_semantic_tokens_full");
    let semantic_tokens = semantic_tokens(&snapshot, &params.text_document.uri)?;
    // Unconditionally cache the tokens
    snapshot.cache_semantic_tokens(params.text_document.uri, semantic_tokens.clone());
    Ok(Some(SemanticTokensResult::Tokens(semantic_tokens)))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    const DEBUG_HANDLE_SEMANTIC_TOKENS_FULL_DELTA: bool = true;
    let current = semantic_tokens(&snapshot, &params.text_document.uri)?;
    if DEBUG_HANDLE_SEMANTIC_TOKENS_FULL_DELTA {
        eprintln!("start handle_semantic_tokens_full_delta");
    }
    let _p = husky_profile_utils::span("handle_semantic_tokens_full_delta");
    // ad hoc
    let now = Instant::now();
    match snapshot.cached_semantic_tokens_entry(params.text_document.uri) {
        Entry::Occupied(mut entry) => {
            let cached_tokens = entry.get();
            if let Some(ref prev_id) = cached_tokens.result_id {
                if prev_id == &params.previous_result_id {
                    let delta = semantic_token_delta(&cached_tokens, &current);
                    entry.insert(current);
                    if DEBUG_HANDLE_SEMANTIC_TOKENS_FULL_DELTA {
                        eprintln!("end handle_semantic_tokens_full_delta");
                        eprintln!("time elapsed: {}", now.elapsed().as_secs());
                    }
                    return Ok(Some(delta.into()));
                }
            }
            entry.insert(current.clone());
        }
        Entry::Vacant(mut entry) => {
            entry.insert(current.clone());
        }
    }
    if DEBUG_HANDLE_SEMANTIC_TOKENS_FULL_DELTA {
        eprintln!("end handle_semantic_tokens_full_delta");
        eprintln!("time elapsed: {}", now.elapsed().as_secs());
    }
    Ok(Some(SemanticTokensFullDeltaResult::Tokens(current)))
}

pub(crate) fn handle_semantic_tokens_range(
    snapshot: AnalyzerDBSnapshot,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    Ok(None)
    // // ad hoc
    // let path = from_lsp_types::path_from_url(&params.text_document.uri)?;
    // let module_path = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    // let semantic_tokens_ext = snapshot.semantic_tokens_ext(module_path)?;
    // p!(params.range);
    // Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
    //     result_id: None,
    //     data: semantic_tokens_ext.to_vec(),
    // })))
}

fn semantic_tokens(snapshot: &salsa::Snapshot<AnalyzerDB>, uri: &Url) -> Result<SemanticTokens> {
    static SEMANTIC_TOKENS_RESULT_ID_NEXT: AtomicU32 = AtomicU32::new(1);
    fn new_result_id() -> u32 {
        SEMANTIC_TOKENS_RESULT_ID_NEXT.fetch_add(1, Ordering::SeqCst)
    }
    let path = from_lsp_types::path_from_url(uri)?;
    let module_path = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    let semantic_tokens = SemanticTokens {
        result_id: Some(new_result_id().to_string()),
        data: snapshot.semantic_tokens_ext(module_path)?.to_vec(),
    };
    Ok(semantic_tokens)
}

fn semantic_token_delta(
    previous: &SemanticTokens,
    current: &SemanticTokens,
) -> lsp_types::SemanticTokensDelta {
    let edits = semantic_tokens::diff_tokens(&previous.data, &current.data);
    lsp_types::SemanticTokensDelta {
        result_id: current.result_id.clone(),
        edits,
    }
}

fn diff_tokens(old: &[SemanticToken], new: &[SemanticToken]) -> Vec<SemanticTokensEdit> {
    let offset = new
        .iter()
        .zip(old.iter())
        .take_while(|&(n, p)| n == p)
        .count();

    let (_, old) = old.split_at(offset);
    let (_, new) = new.split_at(offset);

    let offset_from_end = new
        .iter()
        .rev()
        .zip(old.iter().rev())
        .take_while(|&(n, p)| n == p)
        .count();

    let (old, _) = old.split_at(old.len() - offset_from_end);
    let (new, _) = new.split_at(new.len() - offset_from_end);

    if old.is_empty() && new.is_empty() {
        vec![]
    } else {
        // The lsp data field is actually a byte-diff but we
        // travel in tokens so `start` and `delete_count` are in multiples of the
        // serialized size of `SemanticToken`.
        vec![lsp_types::SemanticTokensEdit {
            start: 5 * offset as u32,
            delete_count: 5 * old.len() as u32,
            data: Some(new.into()),
        }]
    }
}
