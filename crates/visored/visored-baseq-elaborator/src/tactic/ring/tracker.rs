use eterned::db::EternerDb;
use latex_prelude::helper::tracker::LxDocumentBodyInput;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_annotation::annotation::token::VdTokenAnnotation;
use visored_mir_expr::{
    elaborator::{linear::VdMirSequentialElaborator, VdMirTrivialElaborator},
    helpers::tracker::VdMirExprTracker,
};
use visored_models::VdModels;
use visored_syn_expr::vibe::VdSynExprVibe;

pub struct Tracker {}

type Input<'a> = LxDocumentBodyInput<'a>;

impl Tracker {
    pub fn new(
        input: Input,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        models: &VdModels,
        vibe: VdSynExprVibe,
        db: &EternerDb,
    ) -> Self {
        let VdMirExprTracker {
            root_module_path,
            expr_arena,
            stmt_arena,
            symbol_local_defn_storage,
            source_map,
            sem_expr_range_map,
            sem_phrase_range_map,
            sem_clause_range_map,
            sem_sentence_range_map,
            sem_stmt_range_map,
            sem_division_range_map,
            token_storage,
            output: stmts,
            ..
        } = VdMirExprTracker::new(
            input,
            token_annotations,
            space_annotations,
            models,
            vibe,
            db,
            |_| VdMirTrivialElaborator::default(),
        );
        // assert!(stmts.len() > 1);
        Self {}
    }
}

#[test]
fn ring_tactics_tracker_works() {
    fn t(input: &str) {
        use husky_path_utils::HuskyLangDevPaths;
        use latex_vfs::path::LxFilePath;
        use std::path::PathBuf;

        let db = &EternerDb::default();
        let dev_paths = HuskyLangDevPaths::new();
        let file_path = LxFilePath::new(PathBuf::from(file!()), db);
        let input = LxDocumentBodyInput {
            specs_dir: dev_paths.specs_dir(),
            file_path: LxFilePath::new(PathBuf::from(file!()), db),
            content: input,
        };
        let token_annotations = vec![];
        let space_annotations = vec![];
        let models = &VdModels::new();
        let tracker = Tracker::new(
            input,
            &token_annotations,
            &space_annotations,
            models,
            VdSynExprVibe::ROOT_CNL,
            db,
        );
    }

    t(r#"Let $x\in\mathbb{R}$. Then $x^2=x^2$."#);
}
