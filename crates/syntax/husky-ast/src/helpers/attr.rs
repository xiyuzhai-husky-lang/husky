use crate::*;
use husky_entity_path::{AttrPath, AttrRegistry};

impl AstSheet {
    // todo: needs testing
    #[inline(always)]
    pub fn procure_attrs<'a, D, A: smallvec::Array<Item = D>>(
        &self,
        parent: ItemPath,
        attr_parent_ast_idx: AstIdx,
        mut f: impl FnMut(AstIdx, TokenGroupIdx, AttrPath) -> D,
        db: &dyn AstDb,
    ) -> smallvec::SmallVec<A> {
        let mut registry = AttrRegistry::new(parent);
        let mut attrs: smallvec::SmallVec<A> = smallvec::smallvec![];
        for (ast_idx, token_group_idx, ident) in self
            .siblings()
            .iter()
            .filter_map(move |siblings| {
                siblings
                    .contains(attr_parent_ast_idx)
                    .then_some(siblings.start())
            })
            .map(move |siblings_start| {
                (siblings_start..attr_parent_ast_idx)
                    .rev()
                    .map(|ast_idx| (ast_idx, &self[ast_idx]))
                    .take_while(|(_, ast)| match ast {
                        Ast::Sorc { .. } | Ast::Attr { .. } => true,
                        _ => false,
                    })
                    .filter_map(|(ast_idx, ast)| match ast {
                        Ast::Sorc { .. } => None,
                        Ast::Attr {
                            token_group_idx,
                            ident,
                        } => Some((ast_idx, *token_group_idx, *ident)),
                        _ => unreachable!(),
                    })
            })
            .flatten()
        {
            attrs.push(f(ast_idx, token_group_idx, registry.issue(ident, db)))
        }
        attrs
    }
}
