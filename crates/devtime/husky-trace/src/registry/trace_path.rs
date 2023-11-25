use std::marker::PhantomData;
use vec_like::VecPairMap;

pub(crate) struct TracePathRegistry<Essence>
where
    Essence: Eq,
{
    pub(crate) paths_data: VecPairMap<Essence, u8>,
}

impl<Essence> Default for TracePathRegistry<Essence>
where
    Essence: Eq,
{
    fn default() -> Self {
        Self {
            paths_data: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TracePathDisambiguator<Essence>(u8, PhantomData<Essence>);

impl<Essence> std::fmt::Debug for TracePathDisambiguator<Essence> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<PathData> std::hash::Hash for TracePathDisambiguator<PathData> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<Essence> TracePathRegistry<Essence>
where
    Essence: Eq,
{
    pub(crate) fn issue(&mut self, path_data: Essence) -> TracePathDisambiguator<Essence> {
        let next_disambiguator = self.paths_data.get_value_mut_or_insert_default(path_data);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        TracePathDisambiguator(disambiguator, PhantomData)
    }
}
