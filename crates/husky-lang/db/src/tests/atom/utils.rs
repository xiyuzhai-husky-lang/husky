use crate::*;

use atom::{Atom, AtomKind};

pub(super) fn check_atom_kind(source: &'static str, kind: AtomKind) {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text("haha/main.hsk".into(), source.into());

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id).unwrap();
    let nodes = atomized_main_file.nodes();
    should_be!(nodes.len(), 1);
    let node = &nodes[0];
    let atoms = match should_ok!(node.value().as_ref()) {
        atom::AtomicLineGroup::Stmt(stmt) => stmt.atoms.clone(),
        _ => panic!(),
    };
    should_be!(atoms.len(), 1);
    let atom = &atoms[0];
    should_be!(atom.kind, kind);
}

pub(super) fn get_stmt_atoms_in_one_line_group(
    source: &'static str,
) -> (HuskyLangDatabase, Vec<Atom>) {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text("haha/main.hsk".into(), source.into());

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id).unwrap();
    let nodes = atomized_main_file.nodes();
    should_be!(nodes.len(), 1);
    let node = &nodes[0];
    match node.value().as_ref().unwrap() {
        atom::AtomicLineGroup::Stmt(stmt) => (db, stmt.atoms.clone()),
        _ => panic!(),
    }
}
