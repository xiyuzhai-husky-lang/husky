use super::{action::text_edit::MathAstTextEditAction, TexAstSheetTimeCapsule};
use crate::ast::{TexAstData, TexAstIdx};

pub struct TexAstSheetView<'a> {
    time_capsule: &'a mut TexAstSheetTimeCapsule,
}

pub struct TexAstView<'a> {
    ast_idx: TexAstIdx,
    time_capsule: &'a mut TexAstSheetTimeCapsule,
}

impl<'a> TexAstView<'a> {
    pub fn ui(self, ui: &mut egui::Ui) {
        match self.time_capsule.arena()[self.ast_idx] {
            TexAstData::TextEdit { .. } => self.time_capsule.add_event(|event_builder| {
                event_builder.add_action(MathAstTextEditAction::new(self.ast_idx, |text| {
                    ui.text_edit_singleline(text);
                }))
            }),
            TexAstData::Other => todo!(),
        }
    }
}
