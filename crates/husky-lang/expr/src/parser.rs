use core::{iter::Peekable, slice::Iter};

use atom::{AtomQuery, AtomResult, AtomizedText, Bracket, Opr};
use folded::FoldedList;
use text::TextPosition;

use crate::{error::ExprRule, kind::Opn, precedence::Precedence, stack::ExprStack, *};

pub struct ExprParser<'a> {
    db: &'a dyn ExprQuery,
    arena: ExprArena,
    folded_results: FoldedList<ExprResult>,
}

impl<'a> ExprParser<'a> {
    pub(crate) fn new(db: &'a dyn ExprQuery) -> Self {
        Self {
            db,
            arena: ExprArena::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<ExprResult> {
        self.folded_results
    }
}

impl<'a> folded::Parser<'_, AtomResult, AtomizedText, ExprResult, ExprParser<'a>>
    for ExprParser<'a>
{
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn parse(&mut self, atom_result: &atom::AtomResult) -> ExprResult {
        let atoms = atom_result.as_ref()?.atoms();
        let mut atom_iter = atoms.iter().peekable();
        let mut stack = ExprStack::new(&mut self.arena);
        while let Some(atom) = atom_iter.next() {
            match atom.kind {
                atom::AtomKind::Variable(_)
                | atom::AtomKind::Literal(_)
                | atom::AtomKind::Scope(_) => stack.accept_atom_expr(atom.into()),
                atom::AtomKind::Opr(opr) => {
                    if opr == Opr::Bra(Bracket::Par) {
                        if let Some(atom::AtomKind::Opr(Opr::Ket(Bracket::Par))) =
                            atom_iter.peek().map(|atom| atom.kind.clone())
                        {
                            let ket_atom = atom_iter.next().unwrap();
                            stack.accept_empty_parenthesis((atom.range..ket_atom.range).into());
                        }
                    }
                    stack.accept_opr(opr, atom.range);
                }
            }
        }
        return Ok((atom_result.as_ref()?.attr(), stack.finish()));
    }

    fn folded_results(&mut self) -> &mut FoldedList<ExprResult> {
        &mut self.folded_results
    }
}
