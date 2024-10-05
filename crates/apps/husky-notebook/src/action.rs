use crate::hotkey::NotebookHotkeyAction;

pub enum NotebookAction {
    ToggleDocHelpFacade,
}

impl NotebookAction {
    pub(crate) fn from_hotkey_action(
        argument: Option<usize>,
        action: NotebookHotkeyAction,
    ) -> Self {
        match action {
            NotebookHotkeyAction::ToggleDocHelpFacade => NotebookAction::ToggleDocHelpFacade,
        }
    }
}

#[derive(Default)]
pub struct NotebookActionBuffer {}
