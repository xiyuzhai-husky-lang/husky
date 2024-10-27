use super::{action::text_edit::MathAstTextEditAction, LxAstSheetTimeCapsule};
use crate::ast::{math::LxMathAstData, rose::LxRoseAstData, LxAstData, LxAstIdx};

pub struct LxAstSheetView<'a> {
    time_capsule: &'a mut LxAstSheetTimeCapsule,
}

pub struct LxAstView<'a> {
    ast_idx: LxAstIdx,
    time_capsule: &'a mut LxAstSheetTimeCapsule,
}

impl<'a> LxAstView<'a> {
    pub fn ui(self, ui: &mut egui::Ui) {
        match self.time_capsule.arena()[self.ast_idx] {
            LxAstData::Math(LxMathAstData::TextEdit { .. }) => {
                self.time_capsule.add_event(|event_builder| {
                    event_builder.add_action(MathAstTextEditAction::new(self.ast_idx, |text| {
                        ui.text_edit_singleline(text);
                    }))
                })
            }
            LxAstData::Rose(LxRoseAstData::TextEdit { .. }) => {
                self.time_capsule.add_event(|event_builder| {
                    event_builder.add_action(MathAstTextEditAction::new(self.ast_idx, |text| {
                        ui.text_edit_singleline(text);
                    }))
                })
            }
            _ => todo!(),
        }
    }
}
