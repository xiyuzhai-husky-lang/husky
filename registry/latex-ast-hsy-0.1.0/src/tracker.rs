use crate::{file::map::LxFileMap, LxAstId, LxFileIdx, LX_FILE_STORAGE};
use husky_path_utils::HuskyLangDevPaths;
use latex_ast::ast::LxAstIdx;
use latex_ast::helpers::tracker::LxAstTracker;
use latex_prelude::helper::tracker::LxDocumentInput;
use latex_vfs::path::LxFilePath;
use lazy_static::lazy_static;
use std::path::PathBuf;
use vec_like::SmallVecSet;

lazy_static! {
    pub static ref TRACKERS: LxFileMap<LxAstTrackerExtended> = trackers();
}

pub struct LxAstTrackerExtended {
    pub tracker: LxAstTracker<'static, LxDocumentInput<'static>>,
    pub asts_over_tokens: Vec<SmallVecSet<LxAstIdx, 4>>,
}

impl std::ops::Deref for LxAstTrackerExtended {
    type Target = LxAstTracker<'static, LxDocumentInput<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.tracker
    }
}

// TODO: maybe use memo-like abstraction for this?
fn trackers() -> LxFileMap<LxAstTrackerExtended> {
    let dev_paths = HuskyLangDevPaths::new();
    let specs_dir = dev_paths.specs_dir();
    LX_FILE_STORAGE.file_map(|i, content| {
        let file_path = LxFilePath::new(PathBuf::from(format!("lx-file-{i}",)));
        let tracker = LxAstTracker::new(LxDocumentInput {
            specs_dir: specs_dir.to_path_buf(),
            file_path,
            content,
        });
        let asts_over_tokens = calc_asts_over_tokens(&tracker);
        LxAstTrackerExtended {
            tracker,
            asts_over_tokens,
        }
    })
}

fn calc_asts_over_tokens(
    tracker: &LxAstTracker<'static, LxDocumentInput<'static>>,
) -> Vec<SmallVecSet<LxAstIdx, 4>> {
    // TODO: other lanes, and a data structure for token map
    let mut asts_over_tokens: Vec<SmallVecSet<LxAstIdx, 4>> =
        (0..tracker.token_storage.main_ranged_tokens().len())
            .into_iter()
            .map(|_| Default::default())
            .collect();
    let LxAstTracker {
        ast_arena,
        ast_token_idx_range_map,
        ..
    } = tracker;
    for ast_idx in ast_arena.all_asts() {
        let token_idx_range = ast_token_idx_range_map[ast_idx];
        for token_idx in token_idx_range {
            asts_over_tokens[token_idx.index()].insert(ast_idx);
        }
    }
    asts_over_tokens
}

#[test]
fn trackers_works() {
    let trackers = trackers();
    assert_eq!(trackers.len(), LX_FILE_STORAGE.files().len());
}

pub(crate) fn all_asts_within_file(file_idx: LxFileIdx) -> impl Iterator<Item = LxAstId> {
    let tracker = &TRACKERS[file_idx];
    tracker
        .ast_arena
        .all_asts()
        .map(move |ast_idx| LxAstId { file_idx, ast_idx })
}

pub(crate) fn first_ast(file_idx: LxFileIdx) -> LxAstIdx {
    let tracker = &TRACKERS[file_idx];
    tracker.asts_over_tokens[0][0]
}
