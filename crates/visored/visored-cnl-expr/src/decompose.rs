use crate::parser::VdCnlParser;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use is_llm::IsLlm;
use once_place::OncePlace;

pub struct DcnEntry<D: Decompose> {
    natural_form: String,
    data: OncePlace<D>,
}

pub trait Decompose: Sized {
    fn arena_mut<'a>(parser: &'a mut VdCnlParser) -> &'a mut DcnArena<Self>;
}

pub type DcnIdx<D> = ArenaIdx<DcnEntry<D>>;
pub type DcnIdxRange<D> = ArenaIdxRange<DcnEntry<D>>;
pub type DcnArena<D> = Arena<DcnEntry<D>>;
pub type DcnArenaRef<'a, D> = ArenaRef<'a, DcnEntry<D>>;

impl<D> DcnEntry<D>
where
    D: Decompose,
{
    pub fn new(natural_form: String) -> Self {
        Self {
            natural_form,
            data: OncePlace::default(),
        }
    }
}
