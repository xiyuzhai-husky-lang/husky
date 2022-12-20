use super::*;

pub enum Prelude<'a> {
    Ongoing(IdentMap<ModuleItem>),
    Finished(&'a IdentMap<ModuleItem>),
}
