use crate::*;
use husky_atom::context::{AtomContext, AtomContextKind, Symbol};
use husky_token_syntax::AbsSemanticToken;

impl<'a> AtomContext<'a> for AstTransformer<'a> {
    fn opt_target_entrance(&self) -> Option<FileItd> {
        Some(self.main)
    }

    fn entity_syntax_db(&self) -> &'a dyn husky_entity_syntax::EntitySyntaxQueryGroup {
        self.db.upcast()
    }

    fn opt_this_ty(&self) -> Option<Ty> {
        self.opt_base_ty.value()
    }

    fn opt_this_liason(&self) -> Option<ParameterModifier> {
        self.opt_this_liason.value()
    }

    fn symbols(&self) -> &[Symbol] {
        (&self.symbols as &[Symbol]).into()
    }

    fn kind(&self) -> AtomContextKind {
        AtomContextKind::Normal
    }

    fn push_abs_semantic_token(&mut self, new_token: AbsSemanticToken) {
        if self.abs_semantic_tokens.len() > 0 {
            let last_end = self.abs_semantic_tokens.last().unwrap().range.end;
            let new_start = new_token.range.start;
            should!(last_end.i() <= new_start.i());
            if last_end.i() == new_start.i() {
                should!(last_end.j() <= new_start.j());
            }
        }
        self.abs_semantic_tokens.push(new_token)
    }

    fn as_dyn_mut(&mut self) -> &mut dyn AtomContext<'a> {
        self
    }

    fn save_state(&self) -> AtomContextState {
        AtomContextState {
            abs_semantic_tokens_len: self.abs_semantic_tokens.len(),
        }
    }

    fn rollback(&mut self, state: AtomContextState) {
        self.abs_semantic_tokens
            .truncate(state.abs_semantic_tokens_len)
    }

    fn push_symbol(&mut self, new_symbol: Symbol) {
        self.symbols.push(new_symbol)
    }

    fn file(&self) -> FileItd {
        todo!()
    }
}
