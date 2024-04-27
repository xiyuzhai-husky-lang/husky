/// https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html#the-captures-trick
pub trait Captures<'a> {}

impl<'a, T: ?Sized> Captures<'a> for T {}
