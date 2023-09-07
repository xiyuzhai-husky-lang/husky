use crate::*;
use husky_entity_path::{DecrParentPath, DecrPath, DecrRegistry};

impl AstSheet {
    // todo: needs testing
    #[inline(always)]
    pub fn procure_decrs<'a, D, A: smallvec::Array<Item = D>>(
        &self,
        parent: ItemPath,
        decr_parent_ast_idx: AstIdx,
        mut f: impl FnMut(AstIdx, TokenGroupIdx, DecrPath) -> D,
        db: &dyn AstDb,
    ) -> smallvec::SmallVec<A> {
        let mut registry = DecrRegistry::new(parent);
        let mut decrs: smallvec::SmallVec<A> = smallvec::smallvec![];
        for (ast_idx, token_group_idx, ident) in self
            .siblings()
            .iter()
            .filter_map(move |siblings| {
                siblings
                    .contains(decr_parent_ast_idx)
                    .then_some(siblings.start())
            })
            .map(move |siblings_start| {
                (siblings_start..decr_parent_ast_idx)
                    .rev()
                    .map(|ast_idx| (ast_idx, &self[ast_idx]))
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
            decrs.push(f(ast_idx, token_group_idx, registry.issue(ident, db)))
        }
        decrs
    }
}
