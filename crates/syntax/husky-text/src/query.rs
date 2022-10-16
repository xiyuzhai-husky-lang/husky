use crate::*;

use husky_file::{FileItd, FileQueryGroup};

#[salsa::query_group(TextQueryGroupStorage)]
pub trait TextQueryGroup: FileQueryGroup {
    fn text(&self, file: FileItd) -> Option<Arc<HuskyText>>;
}

fn text(this: &dyn TextQueryGroup, file: FileItd) -> Option<Arc<HuskyText>> {
    this.raw_text(file)
        .map(|raw_text| Arc::new(HuskyText::new(&raw_text)))
}
