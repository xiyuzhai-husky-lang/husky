use serde::Serialize;

pub trait IsException:
    std::fmt::Debug + std::fmt::Display + Clone + Eq + Serialize + 'static
{
}
