pub trait NotifyEvent: Send + 'static {
    fn notify_event(&self);
}

impl NotifyEvent for () {
    fn notify_event(&self) {}
}
