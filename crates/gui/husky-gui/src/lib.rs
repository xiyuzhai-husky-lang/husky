mod response;
mod widgets;

pub use self::response::Response;
pub use self::widgets::Widget;

pub struct Ui(egui::Ui);
