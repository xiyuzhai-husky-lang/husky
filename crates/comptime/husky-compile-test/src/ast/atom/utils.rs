#[cfg(test)]
use crate::*;
#[cfg(test)]
use husky_atom::{
    context::{AtomContextKind, Symbol},
    *,
};
#[cfg(test)]
pub(super) fn check_atom_kind(db: &mut Comptime, line: &'static str, kind: HuskyAtomVariant) {
    let atoms = get_atoms_in_line(db, line);
    let atom = &atoms[0];
    should_eq!(atom.variant, kind);
}
#[cfg(test)]
pub(super) fn get_atoms_in_line(db: &mut Comptime, line: &'static str) -> Vec<HuskyAtom> {
    db.set_live_file_text("haha/main.hsy".into(), line.into());
    let tokens = db.tokenize(line);
    let main = db.intern_file("haha/main.hsy".into());
    db.set_opt_target_entrance(Some(main));
    let symbols = fold::LocalStack::new();
    AtomParser::new(
        &mut AtomContextStandalone {
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
