use crate::*;



use husky_token::Token;

impl<'a> AstTransformer<'a> {
    pub fn parse_expr(&mut self, tokens: &[Token]) -> AstResult<RawExprIdx> {
        let atoms = self.parse_atoms(tokens, |parser| parser.parse_all_atoms())?;
        self.parse_expr_from_atoms(atoms)
    }

    pub(crate) fn parse_expr_from_atoms(&mut self, _atoms: Vec<HuskyAtom>) -> AstResult<RawExprIdx> {
        todo!()
        // should!(atoms.len() > 0);
        // let mut atom_iter = atoms.into_iter().peekable();
        // let mut stack = ExprStack::new(&mut self.arena);
        // while let Some(atom) = atom_iter.next() {
        //     let atom_text_start = atom.text_start();
        //     let end = atom.text_end();
        //     match atom.variant {
        //         HuskyAtomVariant::Variable { .. }
        //         | HuskyAtomVariant::ThisValue { .. }
        //         | HuskyAtomVariant::ThisField { .. }
        //         | HuskyAtomVariant::PrimitiveLiteral(_)
        //         | HuskyAtomVariant::EntityRoute { .. }
        //         | HuskyAtomVariant::FrameVariable { .. } => stack.accept_atom_expr(atom.into()),
        //         HuskyAtomVariant::Unrecognized(_) => stack.accept_atom_expr(atom.into()),
        //         HuskyAtomVariant::Binary(opr) => stack.accept_binary(opr)?,
        //         HuskyAtomVariant::Prefix(prefix) => stack.accept_prefix(prefix, atom.text_start()),
        //         HuskyAtomVariant::Suffix(suffix) => stack.accept_suffix(suffix, end),
        //         HuskyAtomVariant::FieldAccess(opt_field_ident) => {
        //             stack.accept_field_access(opt_field_ident, atom.text_end())
        //         }
        //         HuskyAtomVariant::ListStart(bra, attr) => {
        //             stack.accept_list_start(bra, attr, atom_text_start, Default::default())
        //         }
        //         HuskyAtomVariant::ListEnd(ket, attr) => {
        //             stack.accept_list_end(ket, attr, atom.text_end())?
        //         }
        //         HuskyAtomVariant::ListItem => stack.accept_list_item(atom.range.start)?,
        //         HuskyAtomVariant::LambdaHead(ref args) => {
        //             stack.accept_lambda_head(args.clone(), atom.text_start())
        //         }
        //         HuskyAtomVariant::SilentEnd => return err!(format!("unexpected `;`"), atom.range),
        //         HuskyAtomVariant::BePattern(pattern) => {
        //             stack.accept_suffix(RawSuffixOpr::BePattern(pattern), end)
        //         }
        //         HuskyAtomVariant::Be => panic!(),
        //         HuskyAtomVariant::WordPattern { .. } => panic!(),
        //     }
        // }
        // stack.finish()
    }
}
