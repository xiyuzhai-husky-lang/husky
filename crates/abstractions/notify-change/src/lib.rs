pub trait NotifyChange: Clone {
    fn notify(&self);
}

impl NotifyChange for () {
    fn notify(&self) {}
}
