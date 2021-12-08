use crate::*;

#[salsa::query_group(FileQueryGroupStorage)]
pub trait FileQueryGroup: std::fmt::Debug {
    #[salsa::input]
    fn virtual_path_input(&self, id: FileId) -> Arc<VirtualPath>;

    #[salsa::input]
    fn file_content_input(&self, id: FileId) -> FileContent;

    fn file_type_input(&self, id: FileId) -> FileType;
}

fn file_type_input(this: &dyn FileQueryGroup, id: FileId) -> FileType {
    let path = this.virtual_path_input(id);
    match path
        .as_path()
        .expect("msg")
        .file_name()
        .expect("something")
        .to_str()
    {
        Some("main.hsk") => FileType::Main,
        _ => FileType::Other,
    };
    todo!()
}
