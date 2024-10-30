use crate::ast::{parse_latex_input_into_asts, LxAstArena, LxAstIdxRange};
use latex_annotation::test_helpers::example::{LxAnnotationsExample, LX_ANNOTATIONS_EXAMPLES};

#[derive(Debug)]
pub struct LatexAstsExample {
    pub annotations: LxAnnotationsExample,
    pub ast_arena: LxAstArena,
    pub asts: LxAstIdxRange,
}

impl LatexAstsExample {
    pub fn from_annotations(annotations: LxAnnotationsExample, db: &salsa::Db) -> Self {
        let mut ast_arena = LxAstArena::default();
        let asts = parse_latex_input_into_asts(
            db,
            &annotations.input,
            &annotations.annotations,
            annotations.root_mode,
            &mut ast_arena,
        );
        Self {
            annotations,
            ast_arena,
            asts,
        }
    }
}

pub fn lx_asts_examples(db: &salsa::Db) -> Vec<LatexAstsExample> {
    LX_ANNOTATIONS_EXAMPLES
        .iter()
        .cloned()
        .map(|eg| LatexAstsExample::from_annotations(eg, db))
        .collect()
}

#[test]
fn test_lx_asts_examples() {
    use expect_test::expect_file;

    let db = crate::DB::default();
    let examples = lx_asts_examples(&db);
    expect_file!["../../expect-files/examples.txt"].assert_debug_eq(&examples);
}
