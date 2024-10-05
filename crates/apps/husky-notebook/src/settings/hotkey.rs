use super::*;
use ui::hotkey::egui::{HotkeyMap, HotkeyPattern};

#[derive(PartialEq, Eq)]
pub struct HuskyNotebookHotkeySettings {
    main_hotkey_map: HotkeyMap<()>,
}

impl Default for HuskyNotebookHotkeySettings {
    fn default() -> Self {
        Self {
            main_hotkey_map: HotkeyMap::new([]).unwrap(),
        }
    }
}

impl NotebookSettings {
    pub(crate) fn hotkey_map(&self) -> impl Iterator<Item = (&HotkeyPattern, ())> {
        self.hotkey.hotkey_map()
    }
}

impl HuskyNotebookHotkeySettings {
    fn hotkey_map(&self) -> impl Iterator<Item = (&HotkeyPattern, ())> {
        self.main_hotkey_map.into_iter()
    }
}
