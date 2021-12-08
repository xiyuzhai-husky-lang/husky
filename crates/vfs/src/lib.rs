//! virtual file system  

pub use file_query_group::FileQueryGroupStorage;
pub use types::{
    AbsPathBuf, FileContent, FileId, FileInterner, FilePosition, FileRange, FileType, VirtualPath,
};

mod file_query_group;
mod types;

use common::*;

/// wrapper around file query group so that we can set file content from either ide input or disk
pub trait VirtualFileSystem: file_query_group::FileQueryGroup {
    /// virtual, private
    fn _get_file_id_map(&self) -> &mut FileInterner;

    /// final, public
    fn get_or_alloc_file_id(&mut self, path: VirtualPath) -> Result<FileId> {
        return match self._get_file_id_map().get_existing_file_id(&path) {
            Some(id) => Ok(id),
            None => {
                let file_content = load_file_content_from_disk(path.as_path()?)?;
                let id = self._get_file_id_map().issue_file_id(path);
                self.set_file_content_input(id, file_content);
                Ok(id)
            }
        };

        fn load_file_content_from_disk(path: &std::path::Path) -> Result<FileContent> {
            Ok(FileContent::OnDisk(Arc::new(std::fs::read_to_string(
                path,
            )?)))
        }
    }

    /// final, public
    fn set_file_content(&mut self, path: VirtualPath, content: FileContent) {
        let id = self._get_file_id_map().get_or_issue_file_id(path);
        self.set_file_content_input(id, content);
    }

    fn virtual_path(&self, id: FileId) -> Arc<VirtualPath> {
        self.virtual_path_input(id)
    }

    fn file_content(&self, id: FileId) -> FileContent {
        self.file_content_input(id)
    }

    fn file_type(&self, id: FileId) -> FileType {
        self.file_type_input(id)
    }
}
