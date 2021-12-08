//! This module provides an SSR assist. It is not desirable to include this
//! assist in ide_assists because that would require the ide_assists crate
//! depend on the ide_ssr crate.

use husky_lang_db::{
    vfs::FileRange, label::Label, source_change::SourceChange, HuskyLangDatabase,
};
use ide_assists::{Assist, AssistId, AssistKind, AssistResolveStrategy, GroupLabel};

pub(crate) fn ssr_assists(
    db: &HuskyLangDatabase,
    resolve: &AssistResolveStrategy,
    frange: FileRange,
) -> Vec<Assist> {
    let mut ssr_assists = Vec::with_capacity(2);

    let (match_finder, comment_range) = match ide_ssr::ssr_from_comment(db, frange) {
        Some(ssr_data) => ssr_data,
        None => return ssr_assists,
    };
    let id = AssistId("ssr", AssistKind::RefactorRewrite);

    let (source_change_for_file, source_change_for_workspace) = if resolve.should_resolve(&id) {
        let edits = match_finder.edits();

        let source_change_for_file = {
            let text_edit_for_file = edits.get(&frange.file_id).cloned().unwrap_or_default();
            SourceChange::from_text_edit(frange.file_id, text_edit_for_file)
        };

        let source_change_for_workspace = SourceChange::from(match_finder.edits());

        (
            Some(source_change_for_file),
            Some(source_change_for_workspace),
        )
    } else {
        (None, None)
    };

    let assists = vec![
        ("Apply SSR in file", source_change_for_file),
        ("Apply SSR in workspace", source_change_for_workspace),
    ];

    for (label, source_change) in assists.into_iter() {
        let assist = Assist {
            id,
            label: Label::new(label.to_string()),
            group: Some(GroupLabel("Apply SSR".into())),
            target: comment_range,
            source_change,
        };

        ssr_assists.push(assist);
    }

    ssr_assists
}
