use crate::*;

use husky_atom::{
    context::{AtomContextKind, Symbol},
    *,
};

pub(super) fn check_atom_kind(db: &mut HuskyCompileTime, line: &'static str, kind: AtomVariant) {
    let atoms = get_atoms_in_line(db, line);
    let atom = &atoms[0];
    should_eq!(atom.variant, kind);
}

pub(super) fn get_atoms_in_line(db: &mut HuskyCompileTime, line: &'static str) -> Vec<HuskyAtom> {
    db.set_live_file_text("haha/main.hsk".into(), line.into());
    let tokens = db.tokenize(line);
    let main = db.intern_file("haha/main.hsk".into());
    let symbols = fold::LocalStack::new();
    AtomParser::new(
        &mut AtomContextStandalone {
            opt_package_main: Some(main),
            db,
            opt_this_ty: None,
            opt_this_contract: None,
            symbols: (&symbols as &[Symbol]).into(),
            kind: AtomContextKind::Normal,
        },
        &mut tokens.as_slice().into(),
    )
    .parse_all()
    .unwrap()
}
