pub trait IsWidgetText: for<'a> From<&'a str> + From<String> {}

impl IsWidgetText for egui::WidgetText {}
