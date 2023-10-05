pub mod helpers;
mod response;
mod widgets;

pub use self::response::Response;

pub struct Ui(egui::Ui);

impl ui::IsUi for Ui {
    type Response = Response;

    type WidgetText = egui::WidgetText;

    fn label(&mut self, text: impl Into<Self::WidgetText>) -> Self::Response {
        todo!()
    }
}
