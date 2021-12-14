#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Error,
    // We don't actually emit this one yet, but we should at some point.
    // Warning,
    WeakWarning,
}
