use crate::*;

#[salsa::query_group(FileQueryGroupStorage)]
pub trait FileQueryGroup: std::fmt::Debug {
    #[salsa::input]
    fn virtual_path_input(&self, id: SourceFileId) -> Arc<VirtualPath>;

    #[salsa::input]
    fn file_content_input(&self, id: SourceFileId) -> SourceFileContent;

    fn file_type_input(&self, id: SourceFileId) -> SourceFileType;
}

fn file_type_input(this: &dyn FileQueryGroup, id: SourceFileId) -> SourceFileType {
    let path = this.virtual_path_input(id);
    match path
        .as_path()
        .expect("msg")
        .file_name()
        .expect("something")
        .to_str()
    {
        Some("main.hsk") => SourceFileType::Main,
        _ => todo!(),
    };
    todo!()
}
