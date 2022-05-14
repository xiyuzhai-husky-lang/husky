use crate::*;

use atom::{
    symbol::{Symbol, SymbolContextKind},
    *,
};

pub(super) fn check_atom_kind(
    db: &mut HuskyLangCompileTime,
    line: &'static str,
    kind: AtomVariant,
) {
    let atoms = get_atoms_in_line(db, line);
    let atom = &atoms[0];
    should_eq!(atom.kind, kind);
}

pub(super) fn get_atoms_in_line(db: &mut HuskyLangCompileTime, line: &'static str) -> Vec<Atom> {
    db.set_live_file_text("haha/main.hsk".into(), line.into());
    let tokens = db.tokenize(line);
    let main = db.intern_file("haha/main.hsk".into());
    let symbols = fold::LocalStack::new();
    AtomParser::new(
        &SymbolContext {
            opt_package_main: Some(main),
            db,
            opt_this_ty: None,
            opt_this_contract: None,
            symbols: (&symbols as &[Symbol]).into(),
            kind: SymbolContextKind::Normal,
        },
        None,
        &tokens,
    )
    .parse_all()
    .unwrap()
}
