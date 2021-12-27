use atom::{AtomKind, AtomParseResult, AtomicLineGroup, AtomicText};
use folded::{FoldedIdx, FoldedList};

use crate::{expr::ExprStack, *};

pub type AstGenResult = AstResult<Ast>;

pub struct AstGenerator {
    arena: ExprArena,
    folded_results: FoldedList<AstGenResult>,
}

impl AstGenerator {
    pub(crate) fn new() -> Self {
        Self {
            arena: ExprArena::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AstGenResult> {
        self.folded_results
    }
}

impl<'a> folded::Generator<'a, AtomParseResult, AtomicText, AstGenResult> for AstGenerator {
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self, _: FoldedIdx<AstGenResult>) {}

    fn transform(&mut self, atom_result: &atom::AtomParseResult) -> AstGenResult {
        match atom_result.clone()? {
            AtomicLineGroup::Stmt(stmt) => {
                let expr = if stmt.atoms.len() == 0 {
                    None
                    // return Ok((atom_result.as_ref()?.attr(), None));
                } else {
                    let mut atom_iter = stmt.atoms.iter().peekable();
                    let mut stack = ExprStack::new(&mut self.arena);
                    while let Some(atom) = atom_iter.next() {
                        match &atom.kind {
                            AtomKind::Variable(_)
                            | AtomKind::Literal(_)
                            | AtomKind::Scope(_, _) => stack.accept_atom_expr(atom.into()),
                            AtomKind::Binary(opr) => stack.accept_binary(*opr),
                            AtomKind::Prefix(_) => todo!(),
                            AtomKind::Suffix(_) => todo!(),
                            AtomKind::ListStart(_, _) => todo!(),
                            AtomKind::ListEnd(_, _) => todo!(),
                            AtomKind::ListItem => todo!(),
                            AtomKind::LambdaHead(_) => todo!(),
                        }
                    }
                    Some(stack.finish())
                };
                Ast::stmt(&self.arena, &stmt.attr, expr)
            }
            AtomicLineGroup::TypeDef { ident, kind, args } => {
                Ok(Ast::TypeDef { ident, kind, args })
            }
            AtomicLineGroup::MainDef => Ok(Ast::MainDef),
            AtomicLineGroup::FuncDef {
                ident,
                kind,
                args,
                inputs,
                output,
            } => Ok(Ast::FuncDef {
                ident,
                kind,
                args,
                inputs,
                output,
            }),
            AtomicLineGroup::Use { ident, scope } => Ok(Ast::Use { ident, scope }),
            AtomicLineGroup::MembDef { ident, kind } => Ok(Ast::MembDef { ident, kind }),
        }
    }

    fn folded_results_mut(&mut self) -> &mut FoldedList<AstGenResult> {
        &mut self.folded_results
    }
}
