pub trait Notify: Send + Clone + 'static {
    fn notify(&self);
}
