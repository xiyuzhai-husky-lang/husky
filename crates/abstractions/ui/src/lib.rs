pub trait IsUi {
    type Response;
}

pub trait Widget<Ui: IsUi> {
    fn ui(self, ui: &mut Ui) -> Ui::Response;
}
