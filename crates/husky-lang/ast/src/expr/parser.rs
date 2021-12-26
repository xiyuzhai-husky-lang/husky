use atom::{AtomKind, AtomParseResult, AtomicText};
use folded::FoldedList;

use crate::{expr::stack::ExprStack, *};

pub struct ExprParser {
    arena: ExprArena,
    folded_results: FoldedList<ExprResult>,
}

impl ExprParser {
    pub(crate) fn new() -> Self {
        Self {
            arena: ExprArena::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<ExprResult> {
        self.folded_results
    }
}

impl<'a> folded::Generator<'a, AtomParseResult, AtomicText, ExprResult> for ExprParser {
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn transform(&mut self, atom_result: &atom::AtomParseResult) -> ExprResult {
        match atom_result.as_ref()? {
            atom::AtomicLineGroup::Stmt(stmt) => {
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
                            AtomKind::Binary(_) => todo!(),
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
                Ok(Some((stmt.attr.clone(), expr)))
            }
            atom::AtomicLineGroup::Decl(_) => todo!(),
        }
    }

    fn folded_results(&mut self) -> &mut FoldedList<ExprResult> {
        &mut self.folded_results
    }

    fn transform_all(&mut self, mut iter: folded::FoldedIter<'a, AtomParseResult, AtomicText>) {
        let mut child_iter = iter.children();
        while let Some((_, value)) = iter.next() {
            // parse current
            let parse_result = self.transform(value);
            self.folded_results().append(parse_result, iter.next);
            // parse children
            self.enter_fold();
            self.transform_all(child_iter);
            self.exit_fold();
            // reset child iter
            child_iter = iter.children();
        }
    }
}
