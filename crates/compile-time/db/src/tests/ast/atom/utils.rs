use crate::*;

use ast::{Atom, AtomKind, AtomLRParser, SymbolProxy};

pub(super) fn check_atom_kind(db: &mut HuskyLangCompileTime, line: &'static str, kind: AtomKind) {
    let atoms = get_atoms_in_line(db, line);
    let atom = &atoms[0];
    should_eq!(atom.kind, kind);
}

pub(super) fn get_atoms_in_line(db: &mut HuskyLangCompileTime, line: &'static str) -> Vec<Atom> {
    db.set_live_file_text("haha/main.hsk".into(), line.into());
    let tokens = db.tokenize(line);
    let main = db.intern_file("haha/main.hsk".into());
    let symbols = fold::LocalStack::new();
    AtomLRParser::new(
        Some(main),
        SymbolProxy {
            main: Some(main),
            db,
            this_ty: None,
            symbols: &symbols,
        },
        &tokens,
    )
    .parse_all()
    .unwrap()
}
