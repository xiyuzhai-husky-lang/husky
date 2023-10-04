use std::error::Error;

pub trait ErrorLineage: Error {
    fn opt_parent(&self) -> Option<&dyn ErrorLineage>;

    fn is_original(&self) -> bool {
        self.opt_parent().is_none()
    }

    fn is_derived(&self) -> bool {
        self.opt_parent().is_some()
    }
}
