//! virtual file system  

use std::sync::RwLock;

pub use file_query_group::FileQueryGroupStorage;
pub use line_map::LineMap;
pub use package::{PackageId, PackageInterner};
pub use source_file::{
    SourceFileContent, SourceFileId, SourceFileInterner, SourceFilePosition, SourceFileRange,
    SourceFileType,
};
pub use virtual_path::VirtualPath;

mod error;
mod file_query_group;
mod line_map;
mod package;
mod source_file;
mod virtual_path;

use common::*;

/// wrapper around file query group so that we can either set file content from ide input or load from disk automatically
/// basically both an input and a derived value
pub trait VirtualFileSystem:
    file_query_group::FileQueryGroup + salsa::plumbing::SalsaInternalOpns
{
    /// virtual, private
    fn file_interner(&self) -> Arc<RwLock<SourceFileInterner>>;
    fn package_interner(&self) -> Arc<RwLock<PackageInterner>>;

    /// final, public
    fn get_or_alloc_file_id(&mut self, path: VirtualPath) -> Result<SourceFileId> {
        let file_interner_arc = self.file_interner();
        let mut file_interner = file_interner_arc.write().unwrap();
        let existing_file_id = file_interner.get_existing_file_id(&path);
        return match existing_file_id {
            Some(id) => Ok(id),
            None => {
                let file_content = load_file_content_from_disk(path.as_path()?)?;
                let id = file_interner.issue_file_id(path);
                self.set_file_content_input(id, file_content);
                Ok(id)
            }
        };

        fn load_file_content_from_disk(path: &std::path::Path) -> Result<SourceFileContent> {
            Ok(SourceFileContent::OnDisk(Arc::new(
                std::fs::read_to_string(path)?,
            )))
        }
    }

    /// final, public
    fn set_file_content(&mut self, path: VirtualPath, content: SourceFileContent) {
        let file_interner_arc = self.file_interner();
        let mut file_interner = file_interner_arc.write().unwrap();
        let id = file_interner.get_or_issue_file_id(path);
        self.request_cancellation();
        self.set_file_content_input(id, content);
    }

    fn set_live(&mut self, path: VirtualPath, text: String) {
        self.set_file_content(path, SourceFileContent::Live(Arc::new(text)));
    }

    fn virtual_path(&self, id: SourceFileId) -> Arc<VirtualPath> {
        self.virtual_path_input(id)
    }

    fn path(&self, _id: SourceFileId) -> &Path {
        todo!()
    }

    fn file_content(&self, id: SourceFileId) -> SourceFileContent {
        self.file_content_input(id)
    }

    fn file_type(&self, id: SourceFileId) -> SourceFileType {
        self.file_type_input(id)
    }

    fn line_map(&self, _id: SourceFileId) -> Result<LineMap> {
        todo!()
    }
}
