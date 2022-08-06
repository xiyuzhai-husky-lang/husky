use std::{
    collections::HashMap,
    path::{self, Path, PathBuf},
};

use crate::*;
use husky_check_utils::should_eq;
use husky_print_utils::{msg_once, p};
use husky_word::CustomIdentifier;
use indexmap::IndexMap;
use itertools::Itertools;
use path_utils::{parent_module_path, submodule_path};
use sync_utils::{ASafeRwLock, SafeRwLock};

pub trait LiveFiles: AllocateUniqueFile {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<FilePtr, ASafeRwLock<String>>>;
    fn did_change_source(&mut self, id: FilePtr);

    fn set_live_file_text(&mut self, path: PathBuf, text: String) {
        let id = self.intern_file(path);
        msg_once!("maybe need improving");
        self.get_live_files()
            .write(|live_docs| live_docs.insert(id, Arc::new(SafeRwLock::new(text))));
        self.did_change_source(id);
    }

    fn apply_live_file_changes(
        &mut self,
        path: PathBuf,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) {
        let id = self.intern_file(path);
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
pub trait FileSalsaQuery: LiveFiles {
    fn file_content(&self, id: FilePtr) -> FileContent;

    fn crate_entrance(&self, module_file: FilePtr) -> Option<FilePtr>;

    fn parent_module_file(&self, module_file: FilePtr) -> Option<FilePtr>;

    fn submodule_file(&self, module_file: FilePtr, ident: CustomIdentifier) -> Option<FilePtr>;
}

fn file_content(db: &dyn FileSalsaQuery, id: FilePtr) -> FileContent {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.get_live_files()
        .read(|live_docs| match live_docs.get(&id) {
            Some(text) => FileContent::Live(text.clone_to_arc()),
            None => {
                let pth: PathBuf = (*id).into();
                if pth.is_file() {
                    FileContent::OnDisk(Arc::new(std::fs::read_to_string(pth).expect("io failure")))
                } else {
                    FileContent::NonExistent
                }
            }
        })
}

fn crate_entrance(db: &dyn FileSalsaQuery, module_file_id: FilePtr) -> Option<FilePtr> {
    let pth: PathBuf = (*module_file_id).into();
    for ancestor in pth.ancestors() {
        let id = db.intern_file(ancestor.with_file_name("main.hsk"));
        match db.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

fn parent_module_file(db: &dyn FileSalsaQuery, module_file: FilePtr) -> Option<FilePtr> {
    Some(db.intern_file(parent_module_path(&module_file, |file| {
        file_exists(db, file)
    })?))
}

fn submodule_file(
    db: &dyn FileSalsaQuery,
    module_file: FilePtr,
    ident: CustomIdentifier,
) -> Option<FilePtr> {
    Some(db.intern_file(submodule_path(&module_file, &ident, |file| {
        file_exists(db, file)
    })?))
}

fn file_exists(db: &dyn FileSalsaQuery, file: &Path) -> bool {
    let file = db.intern_file(file.to_owned());
    match db.file_content(file) {
        FileContent::OnDisk(_) => true,
        FileContent::Live(_) => true,
        FileContent::Deleted => false,
        FileContent::NonExistent => false,
    }
}

pub trait FileQueryGroup: FileSalsaQuery {
    fn file_exists(&self, file: FilePtr) -> bool {
        match self.file_content(file) {
            FileContent::OnDisk(_) => true,
            FileContent::Live(_) => true,
            FileContent::Deleted => false,
            FileContent::NonExistent => false,
        }
    }

    fn all_main_files(&self) -> Vec<FilePtr> {
        self.file_interner()
            .id_iter()
            .filter_map(|id| self.crate_entrance(id))
            .unique()
            .collect()
    }

    fn unique_main_file(&self) -> FilePtr {
        let all_main_files = self.all_main_files();
        assert_eq!(all_main_files.len(), 1);
        all_main_files[0]
    }

    fn raw_text(&self, file: FilePtr) -> Option<Arc<String>> {
        match self.file_content(file) {
            FileContent::OnDisk(text) => Some(text),
            FileContent::Live(text) => Some(text),
            FileContent::Deleted => None,
            FileContent::NonExistent => None,
        }
    }

    fn url(&self, id: FilePtr) -> lsp_types::Url {
        return url_from_abs_path(&id);

        pub(crate) fn url_from_abs_path(path: &Path) -> lsp_types::Url {
            let url = lsp_types::Url::from_file_path(path).unwrap();
            match path.components().next() {
                Some(path::Component::Prefix(prefix))
                    if matches!(
                        prefix.kind(),
                        path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
                    ) =>
                {
                    // Need to lowercase driver letter
                }
                _ => return url,
            }

            let driver_letter_range = {
                let (scheme, drive_letter, _rest) =
                    match url.as_str().splitn(3, ':').collect_tuple() {
                        Some(it) => it,
                        None => return url,
                    };
                let start = scheme.len() + ':'.len_utf8();
                start..(start + drive_letter.len())
            };

            // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
            // machinery *also* canonicalizes the drive letter. So, just massage the
            // string in place.
            let mut url: String = url.into();
            url[driver_letter_range].make_ascii_lowercase();
            lsp_types::Url::parse(&url).unwrap()
        }
    }
}
