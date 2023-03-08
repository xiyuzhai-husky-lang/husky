use crate::*;

pub fn set_live_file<Db: VfsDb>(db: &mut Db, path: &Path, text: String) -> VfsResult<()> {
    eprintln!("set live file for path {:?}", path);
    update_live_packages(db, path);
    eprintln!("finish update live packages");
    db.set_content(path, FileContent::LiveDoc(text))
}

/// If range are omitted
/// the new text is considered to be the full content of the document.
pub fn apply_live_file_changes<Db: VfsDb>(
    db: &mut Db,
    path: &Path,
    changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> VfsResult<()> {
    update_live_packages(db, path);
    update_content(db, path, |text| {
        husky_text::apply_document_changes(text, changes)
    });
    Ok(())
}

fn update_live_packages(db: &dyn VfsDb, path: &Path) {
    if let Ok(toolchain) = db.current_toolchain() {
        db.resolve_module_path(toolchain, path);
    }
}

fn update_content<T: VfsDb>(db: &mut T, path: &Path, f: impl FnOnce(&mut String)) -> VfsResult<()> {
    let abs_path = DiffPath::try_new(db, path)?;
    let file = match db
        .vfs_jar()
        .cache()
        .files()
        .entry(abs_path.path(db).to_owned())
    {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(_entry) => return Ok(()),
    };
    let mut text = file.text(db).unwrap_or("").to_string();
    f(&mut text);
    file.set_content(db).to(FileContent::LiveDoc(text));
    Ok(())
}
