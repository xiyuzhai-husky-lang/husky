use crate::*;
use atom::{context::SymbolKind, AtomVariant};
use text::TextRanged;
use token::Token;

impl<'a> AstTransformer<'a> {
    pub fn parse_expr(&mut self, tokens: &[Token]) -> AstResult<RawExprIdx> {
        let atoms = self.parse_atoms(tokens, |parser| parser.parse_all())?;
        should!(atoms.len() > 0);
        let mut atom_iter = atoms.into_iter().peekable();
        let mut stack = ExprStack::new(&mut self.arena);
        while let Some(atom) = atom_iter.next() {
            let atom_text_start = atom.text_start();
            match atom.kind {
                AtomVariant::Variable { .. }
                | AtomVariant::ThisValue { .. }
                | AtomVariant::ThisField { .. }
                | AtomVariant::PrimitiveLiteral(_)
                | AtomVariant::EntityRoute { .. }
                | AtomVariant::FrameVariable { .. } => stack.accept_atom_expr(atom.into()),
                AtomVariant::Unrecognized(ident) => stack.accept_atom_expr(
                    match self.symbols.find(|symbol| symbol.ident == ident) {
                        Some(symbol) => match symbol.kind {
                            SymbolKind::EntityRoute(_) => todo!(),
                            SymbolKind::Variable { .. } => todo!(),
                            SymbolKind::FrameVariable { .. } => todo!(),
                            SymbolKind::Unrecognized(_) => todo!(),
                            SymbolKind::ThisValue { .. } => todo!(),
                            SymbolKind::ThisField { .. } => todo!(),
                        },
                        None => atom,
                    }
                    .into(),
                ),
                AtomVariant::Binary(opr) => stack.accept_binary(opr)?,
                AtomVariant::Prefix(prefix) => stack.accept_prefix(prefix, atom.text_start()),
                AtomVariant::Suffix(suffix) => stack.accept_suffix(suffix, atom.text_end()),
                AtomVariant::FieldAccess(field_ident) => {
                    stack.accept_field_access(field_ident, atom.text_end())
                }
                AtomVariant::ListStart(bra, attr) => {
                    stack.accept_list_start(bra, attr, atom_text_start, Vec::new())
                }
                AtomVariant::ListEnd(ket, attr) => {
                    stack.accept_list_end(ket, attr, atom.text_end())?
                }
                AtomVariant::ListItem => stack.accept_list_item(atom.range.start)?,
                AtomVariant::LambdaHead(ref args) => {
                    stack.accept_lambda_head(args.clone(), atom.text_start())
                }
                AtomVariant::SilentEnd => return err!(format!("unexpected `;`"), atom.range),
            }
        }
        stack.finish()
    }
}
