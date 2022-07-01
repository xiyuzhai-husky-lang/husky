use crate::*;

use file::{FilePtr, FileQueryGroup};

#[salsa::query_group(TextQueryGroupStorage)]
pub trait TextQueryGroup: FileQueryGroup {
    fn text(&self, file: FilePtr) -> Option<Arc<HuskyText>>;
}

fn text(this: &dyn TextQueryGroup, file: FilePtr) -> Option<Arc<HuskyText>> {
    this.raw_text(file)
        .map(|raw_text| Arc::new(HuskyText::new(&raw_text)))
}
