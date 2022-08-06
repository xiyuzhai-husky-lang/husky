use crate::*;

use husky_atom::{
    context::{AtomContextKind, Symbol},
    *,
};

pub(super) fn check_atom_kind(db: &mut HuskyComptime, line: &'static str, kind: HuskyAtomVariant) {
    let atoms = get_atoms_in_line(db, line);
    let atom = &atoms[0];
    should_eq!(atom.variant, kind);
}

pub(super) fn get_atoms_in_line(db: &mut HuskyComptime, line: &'static str) -> Vec<HuskyAtom> {
    db.set_live_file_text("haha/main.hsk".into(), line.into());
    let tokens = db.tokenize(line);
    let main = db.intern_file("haha/main.hsk".into());
    let symbols = fold::LocalStack::new();
    AtomParser::new(
        &mut AtomContextStandalone {
            opt_target_entrance: Some(main),
            db,
            opt_this_ty: None,
            opt_this_contract: None,
            symbols: (&symbols as &[Symbol]).into(),
            kind: AtomContextKind::Normal,
            opt_file: None,
        },
        &mut tokens.as_slice().into(),
    )
    .parse_all_remaining_atoms()
    .unwrap()
}
