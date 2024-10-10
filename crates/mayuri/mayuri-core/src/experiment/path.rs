use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExperimentPath {
    src_paths: ExperimentSrcPaths,
}

pub type ExperimentSrcPaths = OrderedVecPairMap<ExperimentSrcDestinationPath, String>;

impl ExperimentPath {
    pub fn new(yaml: &Yaml, nemu_config: &NemuConfig) -> Self {
        Self {
            src_paths: nemu_config
                .src_paths()
                .iter()
                .map(|src_path| {
                    (
                        ExperimentSrcDestinationPath::new(src_path.path().to_string()),
                        src_path.path().to_string(),
                    )
                })
                .chain(
                    yaml["src"]
                        .as_hash()
                        .expect("expected hash")
                        .iter()
                        .map(|(k, v)| {
                            (
                                ExperimentSrcDestinationPath::new(
                                    k.as_str()
                                        .expect("invalid yaml, expected string")
                                        .to_string(),
                                ),
                                v.as_str().expect("invalid, expected string").to_string(),
                            )
                        }),
                )
                .collect(),
        }
    }
}

// ... existing code ...
