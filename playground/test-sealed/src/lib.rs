mod haha {
    #[sealed::sealed]
    pub trait Haha {}
}

// this wouldn't compile
// #[sealed::sealed]
// impl Haha for u64 {}
