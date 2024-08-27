use crate::*;
use salsa::Db;

pub fn set_live_file(db: &mut Db, path: &Path, text: String) -> VfsResult<()> {
    db.resolve_module_path_and_update_live_packages(path)?;
    db.set_content(path, FileContent::LiveDoc(text))
}

/// If range are omitted
/// the new text is considered to be the full content of the document.
pub fn apply_live_file_changes(
    db: &mut Db,
    path: &Path,
    changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> VfsResult<()> {
    db.resolve_module_path_and_update_live_packages(path)?;
    update_content(db, path, |text| {
        husky_text_protocol::change::apply_document_changes(text, changes)
    })?;
    Ok(())
}

fn update_content(db: &mut Db, path: &Path, f: impl FnOnce(&mut String)) -> VfsResult<()> {
    let virtual_path = VirtualPath::try_new(db, path)?;
    let file = match db
        .vfs_jar()
        .cache()
        .files()
        .entry(virtual_path.data(db).to_owned())
    {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(_entry) => return Ok(()),
    };
    let mut text = file.text(db)?.unwrap_or("").to_string();
    f(&mut text);
    let path = virtual_path.data(db);
    let durability = db.calc_durability(path)?;
    file.set_content(db)?.to(FileContent::LiveDoc(text));
    Ok(())
}
