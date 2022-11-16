use crate::*;

use husky_path::{FileQueryGroup, PathItd};

#[salsa::query_group(TextQueryGroupStorage)]
pub trait TextQueryGroup: FileQueryGroup {
    fn text(&self, file: PathItd) -> Option<Arc<HuskyText>>;
}

fn text(this: &dyn TextQueryGroup, file: PathItd) -> Option<Arc<HuskyText>> {
    this.raw_text(file)
        .map(|raw_text| Arc::new(HuskyText::new(&raw_text)))
}
