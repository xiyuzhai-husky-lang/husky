mod builder;
mod sem;

pub trait ToLean: Copy {
    type Target;

    fn to_lean(self, builder: &mut ()) -> Self::Target;
}
