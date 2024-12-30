use eterned::db::EternerDb;
use latex_prelude::helper::tracker::LxDocumentBodyInput;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_annotation::annotation::token::VdTokenAnnotation;
use visored_mir_expr::{
    expr::VdMirExprData, helpers::tracker::VdMirExprTracker, stmt::VdMirStmtData,
    tactic::elaboration::elaborator::VdMirTacticTrivialElaborator,
};
use visored_models::VdModels;
use visored_syn_expr::vibe::VdSynExprVibe;
use visored_tactics_standard_linear_elaborator::{
    session::VdTacticsEvaluationSession,
    tactics::ring::{engine::VdTacticsEvaluationRingEngine, tracker::*, *},
};

fn main() {
    let start = std::time::Instant::now();
    ring_tactics();
    let duration = start.elapsed();
    println!("Ring tactics took: {:?}", duration);
}

fn ring_tactics() {
    fn t(input: &str, repetitions: usize) {
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
            &token_annotations,
            &space_annotations,
            models,
            VdSynExprVibe::ROOT_CNL,
            db,
            VdMirTacticTrivialElaborator::default(), // ad hoc
        );
        let stmt = stmts.last().unwrap();
        let VdMirStmtData::Block { stmts, ref meta } = stmt_arena[stmt] else {
            unreachable!()
        };
        let stmt = stmts.last().unwrap();
        let VdMirStmtData::Block { stmts, ref meta } = stmt_arena[stmt] else {
            unreachable!()
        };
        let stmt = stmts.last().unwrap();
        let VdMirStmtData::Block { stmts, ref meta } = stmt_arena[stmt] else {
            unreachable!()
        };
        let stmt = stmts.last().unwrap();
        let VdMirStmtData::Have { prop, tactics } = stmt_arena[stmt] else {
            unreachable!()
        };
        let VdMirExprData::ChainingSeparatedList {
            leader,
            ref followers,
            ..
        } = expr_arena[prop]
        else {
            unreachable!("expr_arena[prop] = {:?}", expr_arena[prop])
        };
        assert_eq!(followers.len(), 1);
        let lopd = leader;
        let ropd = followers[0].1;
        let sess = VdTacticsEvaluationSession::new(db);
        for _ in 0..repetitions {
            let mut engine = VdTacticsEvaluationRingEngine::new(&sess, expr_arena.as_arena_ref());
            engine.judge(lopd, ropd);
        }
    }

    let inputs = vec![
        r#"Let $x\in\mathbb{R}$. Then $x+1=1+x$."#,
        r#"Let $x\in\mathbb{R}$. Let $y\in\mathbb{R}$. Then $x+1+y=y+1+x$."#,
    ];
    for input in inputs {
        t(&input, 10000);
    }
}
