use crate::*;
use itertools::Itertools;
use stdx::sync::ARwLock;

pub trait LiveFiles: InternFile {
    fn get_live_files(&self) -> &ARwLock<HashMap<FileId, ARwLock<String>>>;
    fn did_change_source(&mut self, id: FileId);

    fn set_live_file_text(&mut self, path: PathBuf, text: String) {
        let id = self.file_id(path);
        self.get_live_files()
            .write(|live_docs| live_docs.insert(id, ARwLock::new(text)));
        self.did_change_source(id);
    }

    fn apply_live_file_changes(
        &mut self,
        path: PathBuf,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) {
        let id = self.file_id(path);
        self.get_live_files().write(|live_docs| {
            live_docs
                .get(&id)
                .expect("what")
                .write(|text| utils::apply_document_changes(text, changes))
        });
        self.did_change_source(id);
    }
}

#[salsa::query_group(FileQueryStorage)]
pub trait FileSalsaQuery: salsa::Database + std::fmt::Debug + LiveFiles {
    fn file_content(&self, id: FileId) -> FileContent;

    fn main_file_id(&self, module_file_id: FileId) -> Option<FileId>;
}

fn file_content(this: &dyn FileSalsaQuery, id: FileId) -> FileContent {
    this.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    this.get_live_files()
        .read(|live_docs| match live_docs.get(&id) {
            Some(text) => FileContent::Live(text.clone_to_arc()),
            None => {
                let pth: PathBuf = use_filepath(this, id, |pth| pth.into());
                if pth.is_file() {
                    FileContent::OnDisk(Arc::new(std::fs::read_to_string(pth).expect("io failure")))
                } else {
                    FileContent::NonExistent
                }
            }
        })
}

fn main_file_id(this: &dyn FileSalsaQuery, module_file_id: FileId) -> Option<FileId> {
    let pth: PathBuf = use_filepath(this, module_file_id, |pth| pth.into());
    for ancestor in pth.ancestors() {
        let id = this.file_id(ancestor.with_file_name("main.hsk"));
        match this.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

pub trait FileQuery: FileSalsaQuery {
    fn file_exists(&self, id: FileId) -> bool {
        match self.file_content(id) {
            FileContent::OnDisk(_) => true,
            FileContent::Live(_) => true,
            FileContent::Deleted => false,
            FileContent::NonExistent => false,
        }
    }

    fn all_main_files(&self) -> Vec<FileId> {
        self.file_id_iter()
            .filter_map(|id| self.main_file_id(id))
            .unique()
            .collect()
    }

    fn text(&self, id: FileId) -> Option<Arc<String>> {
        match self.file_content(id) {
            FileContent::OnDisk(text) => Some(text),
            FileContent::Live(text) => Some(text),
            FileContent::Deleted => None,
            FileContent::NonExistent => None,
        }
    }
}
