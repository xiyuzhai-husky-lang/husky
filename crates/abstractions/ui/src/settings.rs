use crate::ui::IsUi;

pub trait SettingsUi<Ui: IsUi> {
    fn for_each_section(&mut self, f: &mut dyn FnMut(&str, &mut dyn SettingSectionUi<Ui>));
}

pub trait SettingSectionUi<Ui: IsUi> {
    fn for_each_subsection(&mut self, f: &mut dyn FnMut(&str, &mut dyn SettingSubsectionUi<Ui>));
}

/// trivial implementation
impl<Ui: IsUi> SettingSectionUi<Ui> for () {
    fn for_each_subsection(&mut self, f: &mut dyn FnMut(&str, &mut dyn SettingSubsectionUi<Ui>)) {}
}

pub trait SettingSubsectionUi<Ui: IsUi> {
    fn for_each_item(&mut self, f: &mut dyn FnMut(&str, &mut dyn SettingItemUi<Ui>));
}

/// trivial implementation
impl<Ui: IsUi> SettingSubsectionUi<Ui> for () {
    fn for_each_item(&mut self, f: &mut dyn FnMut(&str, &mut dyn SettingItemUi<Ui>)) {}
}

pub trait SettingItemUi<Ui: IsUi> {
    fn setting_item_ui(&mut self, ui: &mut Ui);
}

/// trivial implementation
impl<Ui: IsUi> SettingItemUi<Ui> for () {
    fn setting_item_ui(&mut self, _ui: &mut Ui) {}
}

/// trivial implementation
impl<Ui: IsUi> SettingsUi<Ui> for () {
    fn for_each_section(&mut self, _f: &mut dyn FnMut(&str, &mut dyn SettingSectionUi<Ui>)) {}
}
