pub trait IntoError: Sized {
    type Error: From<Self>;
}

impl IntoError for () {
    type Error = Self;
}
