pub trait NotifyChange: Send + 'static {
    fn notify(&self);
}

impl NotifyChange for () {
    fn notify(&self) {}
}
