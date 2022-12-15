// TODO: CLEAR THIS FILE
use std::path::{self, Path, PathBuf};

use crate::*;
use husky_path_utils::{parent_module_path, submodule_path};
use husky_print_utils::msg_once;
use husky_word::Identifier;
use indexmap::IndexMap;
use itertools::Itertools;
use sync_utils::{ASafeRwLock, SafeRwLock};
use upcast::UpcastMut;

pub trait VfsQueryGroupBase: InternHuskyPath {
    // #[cfg(feature = "lsp_support")]
    #[inline(always)]
    fn get_live_files(&self) -> Option<&ASafeRwLock<IndexMap<AbsolutePath, ASafeRwLock<String>>>>;

    #[inline(always)]
    fn watch(&self, path: AbsolutePath);
}

pub trait LiveFileSupport: FileSalsaQuery + UpcastMut<dyn FileSalsaQuery> {
    fn did_change_source(&mut self, id: AbsolutePath) {
        FileContentQuery
            .in_db_mut(self.upcast_mut())
            .invalidate(&id);
    }

    fn set_live_file_text(&mut self, path: PathBuf, text: String) {
        let id = self.intern_path(path);
        msg_once!("maybe need improving");
        self.get_live_files()
            .unwrap()
            .write(|live_docs| live_docs.insert(id, Arc::new(SafeRwLock::new(text))));
        self.did_change_source(id);
    }

    #[cfg(feature = "lsp_support")]
    fn apply_live_file_changes(
        &mut self,
        path: PathBuf,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) {
        FileContentQuery
            .in_db_mut(self.upcast_mut())
            .invalidate(todo!());
        let id = self.intern_path(path);
        self.get_live_files().unwrap().write(|live_docs| {
            live_docs
                .get(&id)
                .expect("what")
                .write(|text| utils::apply_document_changes(text, changes))
        });
        self.did_change_source(id);
    }
}

#[salsa::query_group(FileQueryStorage)]
pub trait FileSalsaQuery: VfsQueryGroupBase {
    #[salsa::input]
    fn opt_target_entrance(&self) -> Option<AbsolutePath>;

    fn module_target_entrance(&self, module_file: AbsolutePath) -> Option<AbsolutePath>;

    fn file_content(&self, id: AbsolutePath) -> FileContent;

    fn parent_module_file(&self, module_file: AbsolutePath) -> Option<AbsolutePath>;

    fn submodule_file(&self, module_file: AbsolutePath, ident: Identifier) -> Option<AbsolutePath>;
}

fn file_content(db: &dyn FileSalsaQuery, id: AbsolutePath) -> FileContent {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    // #[cfg(feature = "lsp_support")]
    let opt_live_file = if let Some(live_files) = db.get_live_files() {
        live_files.read(|live_docs| {
            live_docs
                .get(&id)
                .map(|text| FileContent::Live(text.clone_to_arc()))
        })
    } else {
        None
    };

    if let Some(live_file) = opt_live_file {
        live_file
    } else {
        let pth: PathBuf = (*id).into();
        if pth.is_file() {
            FileContent::OnDisk(Arc::new(std::fs::read_to_string(pth).expect("io failure")))
        } else {
            FileContent::NonExistent
        }
    }

    // #[cfg(!(feature = "lsp_support"))]
    // todo!()
}

fn module_target_entrance(
    db: &dyn FileSalsaQuery,
    module_file_id: AbsolutePath,
) -> Option<AbsolutePath> {
    let pth: PathBuf = (*module_file_id).into();
    for ancestor in pth.ancestors() {
        let id = db.intern_path(ancestor.with_file_name("main.hsy"));
        match db.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

fn parent_module_file(db: &dyn FileSalsaQuery, module_file: AbsolutePath) -> Option<AbsolutePath> {
    Some(db.intern_path(parent_module_path(&module_file, |file| {
        file_exists(db, file)
    })?))
}

fn submodule_file(
    db: &dyn FileSalsaQuery,
    module_file: AbsolutePath,
    ident: Identifier,
) -> Option<AbsolutePath> {
    Some(db.intern_path(submodule_path(&module_file, &ident, |file| {
        file_exists(db, file)
    })?))
}

fn file_exists(db: &dyn FileSalsaQuery, file: &Path) -> bool {
    let file = db.intern_path(file.to_owned());
    match db.file_content(file) {
        FileContent::OnDisk(_) => true,
        FileContent::Live(_) => true,
        FileContent::Deleted => false,
        FileContent::NonExistent => false,
    }
}

pub trait FileQueryGroup: FileSalsaQuery {
    fn file_exists(&self, file: AbsolutePath) -> bool {
        match self.file_content(file) {
            FileContent::OnDisk(_) => true,
            FileContent::Live(_) => true,
            FileContent::Deleted => false,
            FileContent::NonExistent => false,
        }
    }

    fn all_target_entrances(&self) -> Vec<AbsolutePath> {
        self.path_itr()
            .id_iter()
            .filter_map(|id| self.module_target_entrance(id))
            .unique()
            .collect()
    }

    fn unique_main_file(&self) -> AbsolutePath {
        let all_main_files = self.all_target_entrances();
        assert_eq!(all_main_files.len(), 1);
        all_main_files[0]
    }

    fn raw_text(&self, file: AbsolutePath) -> Option<Arc<String>> {
        match self.file_content(file) {
            FileContent::OnDisk(text) => Some(text),
            FileContent::Live(text) => Some(text),
            FileContent::Deleted => None,
            FileContent::NonExistent => None,
        }
    }

    #[cfg(feature = "lsp_support")]
    fn url(&self, id: AbsolutePath) -> lsp_types::Url {
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
