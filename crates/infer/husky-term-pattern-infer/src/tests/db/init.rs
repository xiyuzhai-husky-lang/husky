use husky_text::TextRange;

use super::*;

impl TermPatternInferFakeDb {
    pub(super) fn init(&mut self) {
        let entity_path_menu = self.entity_path_menu();
        let term_menu = self.term_menu();
    }
}
