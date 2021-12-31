use atom::{AtomKind, AtomParseResult, AtomicLineGroup, AtomicText};
use folded::{FoldedIdx, FoldedList};
use text::TextRanged;

use crate::{expr::ExprStack, *};

pub type AstGenResult = AstResult<Ast>;

pub(crate) struct AtomToAstTransformer {
    arena: ExprArena,
    folded_results: FoldedList<AstGenResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub arena: ExprArena,
    pub folded_results: FoldedList<AstGenResult>,
}

impl AtomToAstTransformer {
    pub(crate) fn new() -> Self {
        Self {
            arena: ExprArena::new(),
            folded_results: FoldedList::new(),
        }
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            arena: self.arena,
            folded_results: self.folded_results,
        }
    }
}

pub struct AstTask {}

impl folded::Transformer<AtomParseResult, AtomicText, AstGenResult, AstTask>
    for AtomToAstTransformer
{
    fn enter(&mut self) {}

    fn exit(&mut self) {}

    fn post_exit(&mut self, task: AstTask) {
        todo!()
    }

    fn transform(
        &mut self,
        _: folded::Indent,
        atom_result: &atom::AtomParseResult,
    ) -> AstGenResult {
        match atom_result.clone()? {
            AtomicLineGroup::Stmt(stmt) => {
                let expr = if stmt.atoms.len() == 0 {
                    None
                } else {
                    let mut atom_iter = stmt.atoms.iter().peekable();
                    let mut stack = ExprStack::new(&mut self.arena);
                    while let Some(atom) = atom_iter.next() {
                        match &atom.kind {
                            AtomKind::Variable(_)
                            | AtomKind::Literal(_)
                            | AtomKind::Scope(_, _) => stack.accept_atom_expr(atom.into()),
                            AtomKind::Binary(opr) => stack.accept_binary(*opr),
                            AtomKind::Prefix(prefix) => {
                                stack.accept_prefix(*prefix, atom.text_end())
                            }
                            AtomKind::Suffix(suffix) => {
                                stack.accept_suffix(*suffix, atom.text_end())
                            }
                            AtomKind::ListStart(bra, attr) => {
                                stack.accept_list_start(*bra, *attr, atom.text_start())
                            }
                            AtomKind::ListEnd(ket, attr) => {
                                stack.accept_list_end(*ket, *attr, atom.text_end())?
                            }
                            AtomKind::ListItem => stack.accept_list_item(),
                            AtomKind::LambdaHead(args) => {
                                stack.accept_lambda_head(args.clone(), atom.text_start())
                            }
                        }
                    }
                    Some(stack.finish())
                };
                Stmt::parse(&self.arena, &stmt.attr, expr)
            }
            AtomicLineGroup::TypeDef { ident, kind, args } => Ok(Ast::TypeDef {
                ident,
                kind,
                generics: args,
            }),
            AtomicLineGroup::MainDef => Ok(Ast::MainDef),
            AtomicLineGroup::FuncDef { kind, decl } => Ok(Ast::FuncDef { kind, decl }),
            AtomicLineGroup::Use { ident, scope } => Ok(Ast::Use { ident, scope }),
            AtomicLineGroup::MembDef { ident, kind } => Ok(Ast::MembDef { ident, kind }),
        }
    }

    fn folded_outputs_mut(&mut self) -> &mut FoldedList<AstGenResult> {
        &mut self.folded_results
    }

    fn designate_task(&self, output: &AstGenResult) -> Option<AstTask> {
        None
    }
}
