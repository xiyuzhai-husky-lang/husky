mod dataset;

pub(crate) use dataset::DatasetConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) dataset: DatasetConfig,
}
