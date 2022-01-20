use crate::*;

#[derive(Clone, Copy)]
pub struct Compiled(pub fn(&mut VirtualStack) -> InterpretResult<()>);

impl std::fmt::Debug for Compiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("Compiled(")?;
        (self.0 as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl PartialEq for Compiled {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as usize) == (other.0 as usize)
    }
}

impl Eq for Compiled {}
