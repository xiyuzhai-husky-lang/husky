pub trait NotifyChange: Send + Clone + 'static {
    fn notify_change(&self);
}

impl NotifyChange for () {
    fn notify_change(&self) {}
}
