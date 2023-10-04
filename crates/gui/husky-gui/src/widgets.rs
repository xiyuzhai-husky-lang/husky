use crate::*;

pub trait Widget {
    fn ui(self, ui: &mut Ui) -> Response;
}
