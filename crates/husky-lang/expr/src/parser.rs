use atom::{AtomKind, AtomResult, AtomizedText, Bracket, Opr};
use folded::FoldedList;

use crate::{stack::ExprStack, *};

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

impl folded::Transformer<'_, AtomResult, AtomizedText, ExprResult, ExprParser> for ExprParser {
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn transform(&mut self, atom_result: &atom::AtomResult) -> ExprResult {
        let atoms = atom_result.as_ref()?.atoms();
        if atoms.len() == 0 {
            return Ok((atom_result.as_ref()?.attr(), None));
        }
        let mut atom_iter = atoms.iter().peekable();
        let mut stack = ExprStack::new(&mut self.arena);
        while let Some(atom) = atom_iter.next() {
            match atom.kind {
                AtomKind::Variable(_) | AtomKind::Literal(_) | AtomKind::Scope(_) => {
                    stack.accept_atom_expr(atom.into())
                }
                AtomKind::Opr(opr) => {
                    if opr == Opr::Bra(Bracket::Par) {
                        if let Some(AtomKind::Opr(Opr::Ket(Bracket::Par))) =
                            atom_iter.peek().map(|atom| atom.kind.clone())
                        {
                            let ket_atom = atom_iter.next().unwrap();
                            stack.accept_empty_parenthesis((atom.range..ket_atom.range).into())?;
                        }
                    }
                    stack.accept_opr(opr, atom.range)?;
                }
            }
        }
        return Ok((atom_result.as_ref()?.attr(), Some(stack.finish())));
    }

    fn folded_results(&mut self) -> &mut FoldedList<ExprResult> {
        &mut self.folded_results
    }
}
