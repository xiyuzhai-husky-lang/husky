pub trait OriginalError: Sized {
    type Error: From<Self>;
}

impl OriginalError for () {
    type Error = Self;
}
