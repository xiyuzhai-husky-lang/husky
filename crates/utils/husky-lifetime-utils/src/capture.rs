/// https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html#the-captures-trick
pub trait Captures<U> {}

impl<T: ?Sized, U> Captures<U> for T {}
