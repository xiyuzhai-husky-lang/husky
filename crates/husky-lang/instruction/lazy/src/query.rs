#[salsa::query_group(LazyInstructionQueryStorage)]
pub trait LazyInstructionQuery: strict_instruction::StrictInstructionQuery {
    fn main_instructions(&self, main_file: file::FileId) -> Vec<LazyInstruction> {}

    // fn inference_table(&self, file: file::FileId) -> ScopeResultArc<InferenceTable>;
}

fn main_instructions(
    this: &dyn LazyInstructionQuery,
    main_file: file::FileId,
) -> Vec<LazyInstruction> {
    todo!()
}

// fn inference_table(
//     this: &dyn LazyInstructionQuery,
//     file: file::FileId,
// ) -> ScopeResultArc<InferenceTable> {
//     let ast_text = this.ast_text(file)?;
//     let mut inferrer = Inferrer::new(this, &ast_text.arena);
//     inferrer.execute_all(ast_text.folded_results.fold_iter(0));
//     Ok(Arc::new(inferrer.finish()))
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyInstruction {}

#[test]
fn haha() {}
