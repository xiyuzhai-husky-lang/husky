use crate::*;

use ast::{AstKind, Atom, AtomKind};

pub(super) fn check_atom_kind(source: &'static str, kind: AtomKind) {
    todo!()
    // let mut db = HuskyLangCompileTime::default();
    // db.set_live_file_text("haha/main.hsk".into(), source.into());

    // let main_file_id = db.intern_file("haha/main.hsk".into());
    // let ast_text = db.ast_text(main_file_id).unwrap();
    // let nodes = ast_text.folded_results.nodes();
    // should_eq!(nodes.len(), 1);
    // let node = &nodes[0];
    // let atoms = match should_ok!(node.value().as_ref()) {
    //     AstKind::Stmt(stmt) => stmt.atoms.clone(),
    //     _ => panic!(),
    // };
    // should_eq!(atoms.len(), 1);
    // let atom = &atoms[0];
    // should_eq!(atom.kind, kind);
}

pub(super) fn get_stmt_atoms_in_one_line_group(
    source: &'static str,
) -> (HuskyLangCompileTime, Vec<Atom>) {
    todo!()
    // let mut db = HuskyLangCompileTime::default();
    // db.set_live_file_text("haha/main.hsk".into(), source.into());

    // let main_file_id = db.intern_file("haha/main.hsk".into());
    // let ast_text = db.ast_text(main_file_id).unwrap();
    // let nodes = ast_text.folded_results.nodes();
    // should_eq!(nodes.len(), 1);
    // let node = &nodes[0];
    // match node.value().as_ref().unwrap().kind {
    //     AstKind::Stmt(stmt) => (db, stmt.atoms.clone()),
    //     _ => panic!(),
    // }
}
