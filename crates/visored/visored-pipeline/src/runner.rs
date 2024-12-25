use crate::{
    input::VdPipelineInput,
    instance::{
        storage::VdPipelineInstanceStorage, VdPipelineInstance, VdPipelineInstanceIdx,
        VdPipelineInstanceIdxRange,
    },
    VdPipelineConfig, VdPipelineConfigData, VdPipelineResult,
};
use alien_seed::AlienSeed;
use eterned::db::EternerDb;
use std::path::Path;
use std::sync::Arc;
use std::{fs, path::PathBuf};

pub struct VdPipelineRunner<'db> {
    db: &'db EternerDb,
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    instance_storage: VdPipelineInstanceStorage,
    instance_files: Vec<VdPipelineInstanceFile>,
    configs: Vec<Arc<VdPipelineConfig>>,
}

pub struct VdPipelineInstanceFile {
    pub path: PathBuf,
    pub instances: Vec<(Arc<VdPipelineInput>, VdPipelineInstanceIdxRange)>,
}

impl<'db> VdPipelineRunner<'db> {
    pub fn new(
        db: &'db EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        config_path: impl AsRef<Path>,
        src_file_paths: impl IntoIterator<Item = PathBuf>,
    ) -> VdPipelineResult<Self> {
        let configs = VdPipelineConfig::from_yaml_file(config_path)?;
        let mut instance_storage = VdPipelineInstanceStorage::new_empty();

        // Collect, sort, and deduplicate paths
        let mut unique_src_file_paths: Vec<PathBuf> = src_file_paths.into_iter().collect();
        unique_src_file_paths.sort();
        unique_src_file_paths.dedup();

        let instance_files = unique_src_file_paths
            .into_iter()
            .map(|path| {
                let examples = VdPipelineInput::read_examples_from_file(&path)?;
                let instances = examples
                    .iter()
                    .map(|input| {
                        (
                            input.clone(),
                            instance_storage.alloc_instances(configs.iter().map(|config| {
                                VdPipelineInstance::new(config.clone(), input.clone())
                            })),
                        )
                    })
                    .collect();
                Ok(VdPipelineInstanceFile { path, instances })
            })
            .collect::<VdPipelineResult<Vec<_>>>()?;

        Ok(Self {
            db,
            tokio_runtime,
            instance_files,
            configs,
            instance_storage,
        })
    }
}

impl<'db> VdPipelineRunner<'db> {
    pub fn instance_files(&self) -> &[VdPipelineInstanceFile] {
        &self.instance_files
    }
}

impl<'db> std::ops::Index<VdPipelineInstanceIdx> for VdPipelineRunner<'db> {
    type Output = VdPipelineInstance;

    fn index(&self, idx: VdPipelineInstanceIdx) -> &Self::Output {
        &self.instance_storage[idx]
    }
}

impl<'db> VdPipelineRunner<'db> {
    pub fn run_all_single_threaded(&mut self, seed: AlienSeed) -> VdPipelineResult<()> {
        for instance in self.instance_storage.all_instances_mut() {
            instance.run(seed, self.db, self.tokio_runtime.clone())?;
        }
        Ok(())
    }

    pub fn run_all_multi_threaded(&mut self, seed: AlienSeed) -> VdPipelineResult<()> {
        use rayon::prelude::*;

        self.instance_storage
            .all_instances_mut()
            .par_iter_mut()
            .try_for_each(|instance| instance.run(seed, self.db, self.tokio_runtime.clone()))?;
        Ok(())
    }
}
