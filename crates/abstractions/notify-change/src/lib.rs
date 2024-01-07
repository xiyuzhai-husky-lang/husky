pub trait NotifyChange: Send + Clone + 'static {
    fn notify_change(&self);
}
