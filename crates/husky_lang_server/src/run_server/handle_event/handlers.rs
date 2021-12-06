//! This module is responsible for implementing handlers for Language Server
//! Protocol. The majority of requests are fulfilled by calling into the
//! `ide` crate.

use std::{
    io::Write as _,
    process::{self, Stdio},
};

use common::*;

use anyhow::Context;
use ide::{
    AnnotationConfig, AssistKind, AssistResolveStrategy, FileID, FilePosition, FileRange,
    HoverAction, HoverGotoTypeData, Query, RangeInfo, SingleResolve, SourceChange, TextEdit,
};
use ide_db::SymbolKind;
use itertools::Itertools;
use lsp_server::ErrorCode;
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeLens, CompletionItem, Diagnostic, DiagnosticTag, DocumentFormattingParams, FoldingRange,
    FoldingRangeParams, HoverContents, Location, LocationLink, NumberOrString, Position,
    PrepareRenameResponse, Range, RenameParams, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SymbolInformation, SymbolTag,
    TextDocumentIdentifier, Url, WorkspaceEdit,
};
use project::Project;
use serde_json::json;
use stdx::{format_to, never};
use vfs::AbsPathBuf;

use crate::{
    config::huskyfmt::HuskyfmtConfig,
    convert::from_lsp_types,
    convert::to_lsp_types,
    diff::diff,
    line_index,
    line_index::LineEndingType,
    lsp_error::LspError,
    lsp_ext::{
        self, InlayHint, InlayHintsParams, PositionOrRange, ViewCrateGraphParams,
        WorkspaceSymbolParams,
    },
    lsp_utils::{all_edits_are_disjoint, invalid_params_error},
    server::Server,
    server_snapshot::ServerSnapshot,
    Result,
};

pub(crate) fn handle_analyzer_status(
    snap: ServerSnapshot,
    params: lsp_ext::AnalyzerStatusParams,
) -> Result<String> {
    let _p = profile::span("handle_analyzer_status");

    let mut buf = String::new();

    let mut file_id = None;
    if let Some(tdi) = params.text_document {
        match from_lsp_types::to_file_id(&snap, &tdi.uri) {
            Ok(it) => file_id = Some(it),
            Err(_) => format_to!(buf, "file {} not found in vfs", tdi.uri),
        }
    }
    buf.push_str("\nAnalysis:\n");
    buf.push_str(
        &snap
            .analysis
            .status(file_id)
            .unwrap_or_else(|_| "Analysis retrieval was cancelled".to_owned()),
    );
    Ok(buf)
}

pub(crate) fn handle_memory_usage(state: &mut Server, _: ()) -> Result<String> {
    let _p = profile::span("handle_memory_usage");
    let mut mem = state.analysis_host.per_query_memory_usage();
    mem.push(("Remaining".into(), profile::memory_usage().allocated));

    let mut out = String::new();
    for (name, bytes) in mem {
        format_to!(out, "{:>8} {}\n", bytes, name);
    }
    Ok(out)
}

pub(crate) fn handle_syntax_tree(
    snap: ServerSnapshot,
    params: lsp_ext::SyntaxTreeParams,
) -> Result<String> {
    let _p = profile::span("handle_syntax_tree");
    let id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let line_index = snap.file_line_index(id)?;
    let text_range = params
        .range
        .map(|r| from_lsp_types::to_text_range(&line_index, r));
    let res = snap.analysis.syntax_tree(id, text_range)?;
    Ok(res)
}

pub(crate) fn handle_view_hir(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<String> {
    let _p = profile::span("handle_view_hir");
    let position = from_lsp_types::to_file_position(&snap, params)?;
    let res = snap.analysis.view_hir(position)?;
    Ok(res)
}

pub(crate) fn handle_view_item_tree(
    snap: ServerSnapshot,
    params: lsp_ext::ViewItemTreeParams,
) -> Result<String> {
    let _p = profile::span("handle_view_item_tree");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let res = snap.analysis.view_item_tree(file_id)?;
    Ok(res)
}

pub(crate) fn handle_selection_range(
    snap: ServerSnapshot,
    params: lsp_types::SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    let _p = profile::span("handle_selection_range");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let line_index = snap.file_line_index(file_id)?;
    let res: Result<Vec<lsp_types::SelectionRange>> = params
        .positions
        .into_iter()
        .map(|position| {
            let offset = from_lsp_types::to_offset(&line_index, position);
            let mut ranges = Vec::new();
            {
                let mut range = TextRange::new(offset, offset);
                loop {
                    ranges.push(range);
                    let frange = FileRange { file_id, range };
                    let next = snap.analysis.extend_selection(frange)?;
                    if next == range {
                        break;
                    } else {
                        range = next
                    }
                }
            }
            let mut range = lsp_types::SelectionRange {
                range: to_lsp_types::range(&line_index, *ranges.last().unwrap()),
                parent: None,
            };
            for &r in ranges.iter().rev().skip(1) {
                range = lsp_types::SelectionRange {
                    range: to_lsp_types::range(&line_index, r),
                    parent: Some(Box::new(range)),
                }
            }
            Ok(range)
        })
        .collect();

    Ok(Some(res?))
}

pub(crate) fn handle_matching_brace(
    snap: ServerSnapshot,
    params: lsp_ext::MatchingBraceParams,
) -> Result<Vec<Position>> {
    let _p = profile::span("handle_matching_brace");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let line_index = snap.file_line_index(file_id)?;
    let res = params
        .positions
        .into_iter()
        .map(|position| {
            let offset = from_lsp_types::to_offset(&line_index, position);
            let offset = match snap
                .analysis
                .matching_brace(FilePosition { file_id, offset })
            {
                Ok(Some(matching_brace_offset)) => matching_brace_offset,
                Err(_) | Ok(None) => offset,
            };
            to_lsp_types::position(&line_index, offset)
        })
        .collect();
    Ok(res)
}

pub(crate) fn handle_join_lines(
    snap: ServerSnapshot,
    params: lsp_ext::JoinLinesParams,
) -> Result<Vec<lsp_types::TextEdit>> {
    todo!()
}

pub(crate) fn handle_on_enter(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<Vec<lsp_ext::SnippetTextEdit>>> {
    let _p = profile::span("handle_on_enter");
    let position = from_lsp_types::to_file_position(&snap, params)?;
    let edit = match snap.analysis.on_enter(position)? {
        None => return Ok(None),
        Some(it) => it,
    };
    let line_index = snap.file_line_index(position.file_id)?;
    let edit = to_lsp_types::snippet_text_edit_vec(&line_index, true, edit);
    Ok(Some(edit))
}

pub(crate) fn handle_on_type_formatting(
    snap: ServerSnapshot,
    params: lsp_types::DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let _p = profile::span("handle_on_type_formatting");
    let mut position = from_lsp_types::to_file_position(&snap, params.text_document_position)?;
    let line_index = snap.file_line_index(position.file_id)?;

    // in `ide`, the `on_type` invariant is that
    // `text.char_at(position) == typed_char`.
    position.offset -= TextSize::of('.');
    let char_typed = params.ch.chars().next().unwrap_or('\0');

    let text = snap.analysis.file_text(position.file_id)?;
    if !text[usize::from(position.offset)..].starts_with(char_typed) {
        // Add `always!` here once VS Code bug is fixed:
        //   https://github.com/husky-lang-server/husky-lang-server/issues/10002
        return Ok(None);
    }

    // We have an assist that inserts ` ` after typing `->` in `fn foo() ->{`,
    // but it requires precise cursor positioning to work, and one can't
    // position the cursor with on_type formatting. So, let's just toggle this
    // feature off here, hoping that we'll enable it one day, 😿.
    if char_typed == '>' {
        return Ok(None);
    }

    let edit = snap.analysis.on_char_typed(position, char_typed)?;
    let edit = match edit {
        Some(it) => it,
        None => return Ok(None),
    };

    // This should be a single-file edit
    let (_, edit) = edit.source_file_edits.into_iter().next().unwrap();

    let change = to_lsp_types::text_edit_vec(&line_index, edit);
    Ok(Some(change))
}

pub(crate) fn handle_document_symbol(
    snap: ServerSnapshot,
    params: lsp_types::DocumentSymbolParams,
) -> Result<Option<lsp_types::DocumentSymbolResponse>> {
    let _p = profile::span("handle_document_symbol");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let line_index = snap.file_line_index(file_id)?;

    let mut parents: Vec<(lsp_types::DocumentSymbol, Option<usize>)> = Vec::new();

    for node in snap.analysis.file_structure(file_id)? {
        parents.push(create_document_symbol_parent(node, &line_index));
    }

    // Builds hierarchy from a flat list, in reverse order (so that indices
    // makes sense)
    let document_symbols = {
        let mut acc = Vec::new();
        while let Some((mut node, parent_idx)) = parents.pop() {
            if let Some(children) = &mut node.children {
                children.reverse();
            }
            let parent = match parent_idx {
                None => &mut acc,
                Some(i) => parents[i].0.children.get_or_insert_with(Vec::new),
            };
            parent.push(node);
        }
        acc.reverse();
        acc
    };

    let res = if snap.config.enable_hierarchical_symbols() {
        document_symbols.into()
    } else {
        let url = to_lsp_types::url(&snap, file_id);
        let mut symbol_information = Vec::<SymbolInformation>::new();
        for symbol in document_symbols {
            flatten_document_symbol(&symbol, None, &url, &mut symbol_information);
        }
        symbol_information.into()
    };
    return Ok(Some(res));

    fn create_document_symbol_parent(
        node: ide::StructureNode,
        line_index: &line_index::LineCollection,
    ) -> (lsp_types::DocumentSymbol, Option<usize>) {
        #[allow(deprecated)]
        (
            lsp_types::DocumentSymbol {
                name: node.label,
                detail: node.detail,
                kind: to_lsp_types::structure_node_kind(node.kind),
                tags: Some(if node.deprecated {
                    vec![SymbolTag::DEPRECATED]
                } else {
                    vec![]
                }),
                deprecated: Some(node.deprecated),
                range: to_lsp_types::range(line_index, node.node_range),
                selection_range: to_lsp_types::range(line_index, node.navigation_range),
                children: None,
            },
            node.parent,
        )
    }

    fn flatten_document_symbol(
        symbol: &lsp_types::DocumentSymbol,
        container_name: Option<String>,
        url: &Url,
        res: &mut Vec<SymbolInformation>,
    ) {
        let mut tags = Vec::new();

        #[allow(deprecated)]
        if let Some(true) = symbol.deprecated {
            tags.push(SymbolTag::DEPRECATED)
        }

        #[allow(deprecated)]
        res.push(SymbolInformation {
            name: symbol.name.clone(),
            kind: symbol.kind,
            tags: Some(tags),
            deprecated: symbol.deprecated,
            location: Location::new(url.clone(), symbol.range),
            container_name,
        });

        for child in symbol.children.iter().flatten() {
            flatten_document_symbol(child, Some(symbol.name.clone()), url, res);
        }
    }
}

pub(crate) fn handle_workspace_symbol(
    snap: ServerSnapshot,
    params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    let _p = profile::span("handle_workspace_symbol");

    let (all_symbols, libs) = decide_search_scope_and_kind(&params, &snap);

    let query = {
        let query: String = params
            .query
            .chars()
            .filter(|&c| c != '#' && c != '*')
            .collect();
        let mut q = Query::new(query);
        if !all_symbols {
            q.only_types();
        }
        if libs {
            q.libs();
        }
        q.limit(128);
        q
    };
    let mut res = exec_query(&snap, query)?;
    if res.is_empty() && !all_symbols {
        let mut query = Query::new(params.query);
        query.limit(128);
        res = exec_query(&snap, query)?;
    }

    return Ok(Some(res));

    fn decide_search_scope_and_kind(
        params: &WorkspaceSymbolParams,
        snap: &ServerSnapshot,
    ) -> (bool, bool) {
        todo!()
    }

    fn exec_query(snap: &ServerSnapshot, query: Query) -> Result<Vec<SymbolInformation>> {
        let mut res = Vec::new();
        for nav in snap.analysis.symbol_search(query)? {
            let container_name = nav.container_name.as_ref().map(|v| v.to_string());

            #[allow(deprecated)]
            let info = SymbolInformation {
                name: nav.name.to_string(),
                kind: nav
                    .kind
                    .map(to_lsp_types::symbol_kind)
                    .unwrap_or(lsp_types::SymbolKind::VARIABLE),
                tags: None,
                location: to_lsp_types::location_from_nav(snap, nav)?,
                container_name,
                deprecated: None,
            };
            res.push(info);
        }
        Ok(res)
    }
}

pub(crate) fn handle_will_rename_files(
    snap: ServerSnapshot,
    params: lsp_types::RenameFilesParams,
) -> Result<Option<lsp_types::WorkspaceEdit>> {
    let _p = profile::span("handle_will_rename_files");

    let source_changes: Vec<SourceChange> = params
        .files
        .into_iter()
        .filter_map(|file_rename| {
            let from = Url::parse(&file_rename.old_uri).ok()?;
            let to = Url::parse(&file_rename.new_uri).ok()?;

            let from_path = from.to_file_path().ok()?;
            let to_path = to.to_file_path().ok()?;

            // Limit to single-level moves for now.
            match (from_path.parent(), to_path.parent()) {
                (Some(p1), Some(p2)) if p1 == p2 => {
                    if from_path.is_dir() {
                        // add '/' to end of url -- from `file://path/to/folder` to `file://path/to/folder/`
                        let mut old_folder_name = from_path.file_stem()?.to_str()?.to_string();
                        old_folder_name.push('/');
                        let from_with_trailing_slash = from.join(&old_folder_name).ok()?;

                        let imitate_from_url = from_with_trailing_slash.join("mod.rs").ok()?;
                        let new_file_name = to_path.file_name()?.to_str()?;
                        Some((
                            snap.url_to_file_id(&imitate_from_url).ok()?,
                            new_file_name.to_string(),
                        ))
                    } else {
                        let old_name = from_path.file_stem()?.to_str()?;
                        let new_name = to_path.file_stem()?.to_str()?;
                        match (old_name, new_name) {
                            ("mod", _) => None,
                            (_, "mod") => None,
                            _ => Some((snap.url_to_file_id(&from).ok()?, new_name.to_string())),
                        }
                    }
                }
                _ => None,
            }
        })
        .filter_map(|(file_id, new_name)| {
            snap.analysis.will_rename_file(file_id, &new_name).ok()?
        })
        .collect();

    // Drop file system edits since we're just renaming things on the same level
    let mut source_changes = source_changes.into_iter();
    let mut source_change = source_changes.next().unwrap_or_default();
    source_change.file_system_edits.clear();
    // no collect here because we want to merge text edits on same file ids
    source_change.extend(source_changes.map(|it| it.source_file_edits).flatten());
    if source_change.source_file_edits.is_empty() {
        Ok(None)
    } else {
        to_lsp_types::workspace_edit(&snap, source_change).map(Some)
    }
}

pub(crate) fn handle_goto_definition(
    snap: ServerSnapshot,
    params: lsp_types::GotoDefinitionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    let _p = profile::span("handle_goto_definition");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position_params)?;
    let nav_info = match snap.analysis.goto_definition(position)? {
        None => return Ok(None),
        Some(it) => it,
    };
    let src = FileRange {
        file_id: position.file_id,
        range: nav_info.range,
    };
    let res = to_lsp_types::goto_definition_response(&snap, Some(src), nav_info.info)?;
    Ok(Some(res))
}

pub(crate) fn handle_goto_declaration(
    snap: ServerSnapshot,
    params: lsp_types::request::GotoDeclarationParams,
) -> Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    let _p = profile::span("handle_goto_declaration");
    let position =
        from_lsp_types::to_file_position(&snap, params.text_document_position_params.clone())?;
    let nav_info = match snap.analysis.goto_declaration(position)? {
        None => return handle_goto_definition(snap, params),
        Some(it) => it,
    };
    let src = FileRange {
        file_id: position.file_id,
        range: nav_info.range,
    };
    let res = to_lsp_types::goto_definition_response(&snap, Some(src), nav_info.info)?;
    Ok(Some(res))
}

pub(crate) fn handle_goto_implementation(
    snap: ServerSnapshot,
    params: lsp_types::request::GotoImplementationParams,
) -> Result<Option<lsp_types::request::GotoImplementationResponse>> {
    let _p = profile::span("handle_goto_implementation");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position_params)?;
    let nav_info = match snap.analysis.goto_implementation(position)? {
        None => return Ok(None),
        Some(it) => it,
    };
    let src = FileRange {
        file_id: position.file_id,
        range: nav_info.range,
    };
    let res = to_lsp_types::goto_definition_response(&snap, Some(src), nav_info.info)?;
    Ok(Some(res))
}

pub(crate) fn handle_goto_type_definition(
    snap: ServerSnapshot,
    params: lsp_types::request::GotoTypeDefinitionParams,
) -> Result<Option<lsp_types::request::GotoTypeDefinitionResponse>> {
    let _p = profile::span("handle_goto_type_definition");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position_params)?;
    let nav_info = match snap.analysis.goto_type_definition(position)? {
        None => return Ok(None),
        Some(it) => it,
    };
    let src = FileRange {
        file_id: position.file_id,
        range: nav_info.range,
    };
    let res = to_lsp_types::goto_definition_response(&snap, Some(src), nav_info.info)?;
    Ok(Some(res))
}

pub(crate) fn handle_parent_module(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_runnables(
    snap: ServerSnapshot,
    params: lsp_ext::RunnablesParams,
) -> Result<Vec<lsp_ext::Runnable>> {
    todo!()
}

pub(crate) fn handle_related_tests(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<Vec<lsp_ext::TestInfo>> {
    todo!()
}

pub(crate) fn handle_completion(
    snap: ServerSnapshot,
    params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    todo!()
}

pub(crate) fn handle_completion_resolve(
    snap: ServerSnapshot,
    mut original_completion: CompletionItem,
) -> Result<CompletionItem> {
    todo!()
}

pub(crate) fn handle_folding_range(
    snap: ServerSnapshot,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    let _p = profile::span("handle_folding_range");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let folds = snap.analysis.folding_ranges(file_id)?;
    let text = snap.analysis.file_text(file_id)?;
    let line_index = snap.file_line_index(file_id)?;
    let line_folding_only = snap.config.line_folding_only();
    let res = folds
        .into_iter()
        .map(|it| to_lsp_types::folding_range(&*text, &line_index, line_folding_only, it))
        .collect();
    Ok(Some(res))
}

pub(crate) fn handle_signature_help(
    snap: ServerSnapshot,
    params: lsp_types::SignatureHelpParams,
) -> Result<Option<lsp_types::SignatureHelp>> {
    todo!()
}

pub(crate) fn handle_hover(
    snap: ServerSnapshot,
    params: lsp_ext::HoverParams,
) -> Result<Option<lsp_ext::Hover>> {
    Ok(None)
}

pub(crate) fn handle_prepare_rename(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<PrepareRenameResponse>> {
    todo!()
}

pub(crate) fn handle_rename(
    snap: ServerSnapshot,
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    todo!()
}

pub(crate) fn handle_references(
    snap: ServerSnapshot,
    params: lsp_types::ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let _p = profile::span("handle_references");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position)?;

    let refs = match snap.analysis.find_all_refs(position, None)? {
        None => return Ok(None),
        Some(refs) => refs,
    };

    let include_declaration = params.context.include_declaration;
    let locations = refs
        .into_iter()
        .flat_map(|refs| {
            let decl = if include_declaration {
                refs.declaration.map(|decl| FileRange {
                    file_id: decl.nav.file_id,
                    range: decl.nav.focus_or_full_range(),
                })
            } else {
                None
            };
            refs.references
                .into_iter()
                .flat_map(|(file_id, refs)| {
                    refs.into_iter()
                        .map(move |(range, _)| FileRange { file_id, range })
                })
                .chain(decl)
        })
        .filter_map(|frange| to_lsp_types::location(&snap, frange).ok())
        .collect();

    Ok(Some(locations))
}

pub(crate) fn handle_formatting(
    snap: ServerSnapshot,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let _p = profile::span("handle_formatting");

    run_huskyfmt(&snap, params.text_document, None)
}

pub(crate) fn handle_range_formatting(
    snap: ServerSnapshot,
    params: lsp_types::DocumentRangeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let _p = profile::span("handle_range_formatting");

    run_huskyfmt(&snap, params.text_document, Some(params.range))
}

pub(crate) fn handle_code_action(
    snap: ServerSnapshot,
    params: lsp_types::CodeActionParams,
) -> Result<Option<Vec<lsp_ext::CodeAction>>> {
    todo!()
}

pub(crate) fn handle_code_action_resolve(
    snap: ServerSnapshot,
    mut code_action: lsp_ext::CodeAction,
) -> Result<lsp_ext::CodeAction> {
    todo!()
}

fn parse_action_id(action_id: &str) -> Result<(usize, SingleResolve), String> {
    let id_parts = action_id.split(':').collect_vec();
    match id_parts.as_slice() {
        [assist_id_string, assist_kind_string, index_string] => {
            let assist_kind: AssistKind = assist_kind_string.parse()?;
            let index: usize = match index_string.parse() {
                Ok(index) => index,
                Err(e) => return Err(format!("Incorrect index string: {}", e)),
            };
            Ok((
                index,
                SingleResolve {
                    assist_id: assist_id_string.to_string(),
                    assist_kind,
                },
            ))
        }
        _ => Err("Action id contains incorrect number of segments".to_string()),
    }
}

pub(crate) fn handle_code_lens(
    snap: ServerSnapshot,
    params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    Ok(None)
}

pub(crate) fn handle_code_lens_resolve(
    snap: ServerSnapshot,
    code_lens: CodeLens,
) -> Result<CodeLens> {
    let annotation = from_lsp_types::annotation(&snap, code_lens.clone())?;
    let annotation = snap.analysis.resolve_annotation(annotation)?;

    let mut acc = Vec::new();
    to_lsp_types::code_lens(&mut acc, &snap, annotation)?;

    let res = match acc.pop() {
        Some(it) if acc.is_empty() => it,
        _ => {
            never!();
            code_lens
        }
    };

    Ok(res)
}

pub(crate) fn handle_document_highlight(
    snap: ServerSnapshot,
    params: lsp_types::DocumentHighlightParams,
) -> Result<Option<Vec<lsp_types::DocumentHighlight>>> {
    let _p = profile::span("handle_document_highlight");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position_params)?;
    let line_index = snap.file_line_index(position.file_id)?;

    let refs = match snap
        .analysis
        .highlight_related(snap.config.highlight_related(), position)?
    {
        None => return Ok(None),
        Some(refs) => refs,
    };
    let res = refs
        .into_iter()
        .map(
            |ide::HighlightedRange { range, category }| lsp_types::DocumentHighlight {
                range: to_lsp_types::range(&line_index, range),
                kind: category.map(to_lsp_types::document_highlight_kind),
            },
        )
        .collect();
    Ok(Some(res))
}

pub(crate) fn handle_ssr(
    snap: ServerSnapshot,
    params: lsp_ext::SsrParams,
) -> Result<lsp_types::WorkspaceEdit> {
    let _p = profile::span("handle_ssr");
    let selections = params
        .selections
        .iter()
        .map(|range| {
            from_lsp_types::file_range(&snap, params.position.text_document.clone(), *range)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let position = from_lsp_types::to_file_position(&snap, params.position)?;
    let source_change = snap.analysis.structural_search_replace(
        &params.query,
        params.parse_only,
        position,
        selections,
    )??;
    to_lsp_types::workspace_edit(&snap, source_change)
}

pub(crate) fn gen_lsp_diagnostics(
    snapshot: &ServerSnapshot,
    file_id: FileID,
) -> Result<Vec<Diagnostic>> {
    let _p = profile::span("publish_diagnostics");
    let line_index = snapshot.file_line_index(file_id)?;

    let diagnostics: Vec<Diagnostic> = snapshot
        .analysis
        .diagnostics(file_id)?
        .into_iter()
        .map(|d| Diagnostic {
            range: to_lsp_types::range(&line_index, d.range),
            severity: Some(to_lsp_types::diagnostic_severity(d.severity)),
            code: Some(NumberOrString::String(d.code.as_str().to_string())),
            code_description: Some(lsp_types::CodeDescription {
                href: lsp_types::Url::parse(&format!(
                    "https://husky-lang-server.github.io/manual.html#{}",
                    d.code.as_str()
                ))
                .unwrap(),
            }),
            source: Some("husky-lang-server".to_string()),
            message: d.message,
            related_information: None,
            tags: None,
            data: None,
        })
        .collect();
    Ok(diagnostics)
}

pub(crate) fn handle_inlay_hints(
    snap: ServerSnapshot,
    params: InlayHintsParams,
) -> Result<Vec<InlayHint>> {
    todo!()
}

pub(crate) fn handle_call_hierarchy_prepare(
    snap: ServerSnapshot,
    params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    let _p = profile::span("handle_call_hierarchy_prepare");
    let position = from_lsp_types::to_file_position(&snap, params.text_document_position_params)?;

    let nav_info = match snap.analysis.call_hierarchy(position)? {
        None => return Ok(None),
        Some(it) => it,
    };

    let RangeInfo {
        range: _,
        info: navs,
    } = nav_info;
    let res = navs
        .into_iter()
        .filter(|it| it.kind == Some(SymbolKind::Function))
        .map(|it| to_lsp_types::call_hierarchy_item(&snap, it))
        .collect::<Result<Vec<_>>>()?;

    Ok(Some(res))
}

pub(crate) fn handle_call_hierarchy_incoming(
    snap: ServerSnapshot,
    params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    let _p = profile::span("handle_call_hierarchy_incoming");
    let item = params.item;

    let doc = TextDocumentIdentifier::new(item.uri);
    let frange = from_lsp_types::file_range(&snap, doc, item.selection_range)?;
    let fpos = FilePosition {
        file_id: frange.file_id,
        offset: frange.range.start(),
    };

    let call_items = match snap.analysis.incoming_calls(fpos)? {
        None => return Ok(None),
        Some(it) => it,
    };

    let mut res = vec![];

    for call_item in call_items.into_iter() {
        let file_id = call_item.target.file_id;
        let line_index = snap.file_line_index(file_id)?;
        let item = to_lsp_types::call_hierarchy_item(&snap, call_item.target)?;
        res.push(CallHierarchyIncomingCall {
            from: item,
            from_ranges: call_item
                .ranges
                .into_iter()
                .map(|it| to_lsp_types::range(&line_index, it))
                .collect(),
        });
    }

    Ok(Some(res))
}

pub(crate) fn handle_call_hierarchy_outgoing(
    snap: ServerSnapshot,
    params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    let _p = profile::span("handle_call_hierarchy_outgoing");
    let item = params.item;

    let doc = TextDocumentIdentifier::new(item.uri);
    let frange = from_lsp_types::file_range(&snap, doc, item.selection_range)?;
    let fpos = FilePosition {
        file_id: frange.file_id,
        offset: frange.range.start(),
    };

    let call_items = match snap.analysis.outgoing_calls(fpos)? {
        None => return Ok(None),
        Some(it) => it,
    };

    let mut res = vec![];

    for call_item in call_items.into_iter() {
        let file_id = call_item.target.file_id;
        let line_index = snap.file_line_index(file_id)?;
        let item = to_lsp_types::call_hierarchy_item(&snap, call_item.target)?;
        res.push(CallHierarchyOutgoingCall {
            to: item,
            from_ranges: call_item
                .ranges
                .into_iter()
                .map(|it| to_lsp_types::range(&line_index, it))
                .collect(),
        });
    }

    Ok(Some(res))
}

pub(crate) fn handle_semantic_tokens_full(
    snap: ServerSnapshot,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    let _p = profile::span("handle_semantic_tokens_full");

    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let text = snap.analysis.file_text(file_id)?;
    let line_index = snap.file_line_index(file_id)?;

    let highlights = snap.analysis.highlight(file_id)?;
    let highlight_strings = snap.config.highlighting_strings();
    let semantic_tokens =
        to_lsp_types::to_semantic_tokens(&text, &line_index, highlights, highlight_strings);

    // Unconditionally cache the tokens
    snap.semantic_tokens_cache
        .lock()
        .insert(params.text_document.uri, semantic_tokens.clone());

    Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    snap: ServerSnapshot,
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    todo!()
}

pub(crate) fn handle_semantic_tokens_range(
    snapshot: ServerSnapshot,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    let _p = profile::span("handle_semantic_tokens_range");

    let frange = from_lsp_types::file_range(&snapshot, params.text_document, params.range)?;
    let text = snapshot.analysis.file_text(frange.file_id)?;
    let line_index = snapshot.file_line_index(frange.file_id)?;

    let highlights = snapshot.analysis.highlight_range(frange)?;
    let highlight_strings = snapshot.config.highlighting_strings();
    let semantic_tokens =
        to_lsp_types::to_semantic_tokens(&text, &line_index, highlights, highlight_strings);
    Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_open_docs(
    snap: ServerSnapshot,
    params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::Url>> {
    let _p = profile::span("handle_open_docs");
    let position = from_lsp_types::to_file_position(&snap, params)?;

    let remote = snap.analysis.external_docs(position)?;

    Ok(remote.and_then(|remote| Url::parse(&remote).ok()))
}

pub(crate) fn handle_open_cargo_toml(
    snap: ServerSnapshot,
    params: lsp_ext::OpenCargoTomlParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_move_item(
    snap: ServerSnapshot,
    params: lsp_ext::MoveItemParams,
) -> Result<Vec<lsp_ext::SnippetTextEdit>> {
    let _p = profile::span("handle_move_item");
    let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    let range = from_lsp_types::file_range(&snap, params.text_document, params.range)?;

    let direction = match params.direction {
        lsp_ext::MoveItemDirection::Up => ide::Direction::Up,
        lsp_ext::MoveItemDirection::Down => ide::Direction::Down,
    };

    match snap.analysis.move_item(range, direction)? {
        Some(text_edit) => {
            let line_index = snap.file_line_index(file_id)?;
            Ok(to_lsp_types::snippet_text_edit_vec(
                &line_index,
                true,
                text_edit,
            ))
        }
        None => Ok(vec![]),
    }
}

fn to_command_link(command: lsp_types::Command, tooltip: String) -> lsp_ext::CommandLink {
    lsp_ext::CommandLink {
        tooltip: Some(tooltip),
        command,
    }
}

fn show_impl_command_link(
    snap: &ServerSnapshot,
    position: &FilePosition,
) -> Option<lsp_ext::CommandLinkGroup> {
    todo!()
}

fn show_ref_command_link(
    snap: &ServerSnapshot,
    position: &FilePosition,
) -> Option<lsp_ext::CommandLinkGroup> {
    todo!()
}

fn goto_type_action_links(
    snap: &ServerSnapshot,
    nav_targets: &[HoverGotoTypeData],
) -> Option<lsp_ext::CommandLinkGroup> {
    todo!()
}

fn prepare_hover_actions(
    snap: &ServerSnapshot,
    actions: &[HoverAction],
) -> Vec<lsp_ext::CommandLinkGroup> {
    actions
        .iter()
        .filter_map(|it| match it {
            HoverAction::Implementation(position) => show_impl_command_link(snap, position),
            HoverAction::Reference(position) => show_ref_command_link(snap, position),
            HoverAction::GoToType(targets) => goto_type_action_links(snap, targets),
        })
        .collect()
}

fn run_huskyfmt(
    snap: &ServerSnapshot,
    text_document: TextDocumentIdentifier,
    range: Option<lsp_types::Range>,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    todo!()
}
