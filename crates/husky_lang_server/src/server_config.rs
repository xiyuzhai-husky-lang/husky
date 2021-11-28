use vfs::AbsPathBuf;

pub struct ServerConfig {}

impl ServerConfig {
    pub fn linked_projects(&self) -> Vec<LinkedProject> {
        todo!()
    }

    pub fn detached_files(&self) -> &[AbsPathBuf] {
        todo!()
    }
}

enum LinkedProject {}
