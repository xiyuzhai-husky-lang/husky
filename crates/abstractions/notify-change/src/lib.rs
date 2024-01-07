pub trait NotifyChange: Send + Clone + 'static {
    fn notify(&self);
}
