pub trait PureForm {
    type PureForm<'a>: Copy;
}

impl<T> PureForm for T
where
    T: Copy,
{
    type PureForm<'a> = T;
}
