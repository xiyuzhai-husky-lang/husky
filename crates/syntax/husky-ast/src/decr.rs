use crate::*;
use husky_entity_path::{DecrParent, DecrPath, DecrRegistry};

impl AstSheet {
    // todo: needs testing
    #[inline(always)]
    pub fn gen_decrs<'a, D, E>(
        &self,
        target: AstIdx,
        decr_parent: DecrParent,
        f: impl Fn(AstIdx, TokenGroupIdx, DecrPath) -> Result<D, E>,
        invalid_parent: impl FnOnce() -> E,
        db: &dyn AstDb,
    ) -> Result<Vec<D>, E> {
        let mut registry = DecrRegistry::new(decr_parent);
        let mut decrs: Vec<D> = vec![];
        for (ast_idx, token_group_idx, ident) in self
            .siblings
            .iter()
            .filter_map(move |siblings| siblings.contains(target).then_some(siblings.start()))
            .map(move |siblings_start| {
                (siblings_start..target)
                    .rev()
                    .map(|ast_idx| (ast_idx, &self.ast_arena[ast_idx]))
                    .take_while(|(_, ast)| match ast {
                        Ast::Sorc { .. } | Ast::Decr { .. } => true,
                        _ => false,
                    })
                    .filter_map(|(ast_idx, ast)| match ast {
                        Ast::Sorc { .. } => None,
                        Ast::Decr {
                            token_group_idx,
                            ident,
                        } => Some((ast_idx, *token_group_idx, *ident)),
                        _ => unreachable!(),
                    })
            })
            .flatten()
        {
            decrs.push(f(ast_idx, token_group_idx, registry.issue(ident, db))?)
        }
        Ok(decrs)
    }
}
