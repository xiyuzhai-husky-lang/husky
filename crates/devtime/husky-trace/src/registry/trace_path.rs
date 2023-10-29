use std::marker::PhantomData;
use vec_like::VecPairMap;

pub(crate) struct TracePathRegistry<PathData>
where
    PathData: Eq,
{
    pub(crate) paths_data: VecPairMap<PathData, u8>,
}

impl<PathData> Default for TracePathRegistry<PathData>
where
    PathData: Eq,
{
    fn default() -> Self {
        Self {
            paths_data: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TracePathDisambiguator<PathData>(u8, PhantomData<PathData>);

impl<PathData> std::hash::Hash for TracePathDisambiguator<PathData> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<PathData> TracePathRegistry<PathData>
where
    PathData: Eq,
{
    pub(crate) fn issue(&mut self, path_data: PathData) -> TracePathDisambiguator<PathData> {
        let next_disambiguator = self.paths_data.get_value_mut_or_insert_default(path_data);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        TracePathDisambiguator(disambiguator, PhantomData)
    }
}
