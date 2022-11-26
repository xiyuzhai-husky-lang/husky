use crate::*;

use husky_source_path::SourcePath;
use husky_vfs::VfsDb;
use salsa::DbWithJar;

pub trait TextDb: DbWithJar<TextJar> + VfsDb {
    fn text(&self, file: SourcePath) -> Option<Arc<Text>>;
}

impl<T> TextDb for T
where
    T: DbWithJar<TextJar> + VfsDb,
{
    fn text(&self, file: SourcePath) -> Option<Arc<Text>> {
        todo!()
        // this.raw_text(file)
        //     .map(|raw_text| Arc::new(HuskyText::new(&raw_text)))
    }
}
