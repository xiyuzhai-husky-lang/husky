use crate::*;

use file::FilePtr;

pub trait RawTextQueryGroup {
    fn raw_text(&self, file: FilePtr) -> Option<Arc<String>>;
}

#[salsa::query_group(TextQueryGroupStorage)]
pub trait TextQueryGroup: RawTextQueryGroup {
    fn text(&self, file: FilePtr) -> Option<Arc<Text>>;
}

fn text(this: &dyn TextQueryGroup, file: FilePtr) -> Option<Arc<Text>> {
    this.raw_text(file)
        .map(|raw_text| Arc::new(Text::new(&raw_text)))
}
