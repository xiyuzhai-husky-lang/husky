use crate::{
    input::VdPipelineInput,
    instance::{
        storage::VdPipelineInstanceStorage, VdPipelineInstance, VdPipelineInstanceIdx,
        VdPipelineInstanceIdxRange,
    },
    VdPipelineConfig, VdPipelineConfigData, VdPipelineResult,
};
use eterned::db::EternerDb;
use std::path::Path;
use std::sync::Arc;
use std::{fs, path::PathBuf};

pub struct VdPipelineRunner<'db> {
    db: &'db EternerDb,
    instance_storage: VdPipelineInstanceStorage,
    instances_per_file: Vec<(
        PathBuf,
        Vec<(Arc<VdPipelineInput>, VdPipelineInstanceIdxRange)>,
    )>,
    configs: Vec<Arc<VdPipelineConfig>>,
}

impl<'db> VdPipelineRunner<'db> {
    pub fn new(
        db: &'db EternerDb,
        config_path: impl AsRef<Path>,
        src_file_paths: impl IntoIterator<Item = PathBuf>,
    ) -> VdPipelineResult<Self> {
        let configs = VdPipelineConfig::from_yaml_file(config_path)?;
        let mut instance_storage = VdPipelineInstanceStorage::new_empty();
        let instances_per_file: Vec<(
            PathBuf,
            Vec<(Arc<VdPipelineInput>, VdPipelineInstanceIdxRange)>,
        )> = src_file_paths
            .into_iter()
            .map(
                |path| -> VdPipelineResult<(
                    PathBuf,
                    Vec<(Arc<VdPipelineInput>, VdPipelineInstanceIdxRange)>,
                )> {
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
                        .collect::<Vec<_>>();
                    Ok((path, instances))
                },
            )
            .collect::<VdPipelineResult<Vec<_>>>()?;

        Ok(Self {
            db,
            instances_per_file,
            configs,
            instance_storage,
        })
    }
}

impl<'db> VdPipelineRunner<'db> {
    pub fn run_all_single_threaded(&mut self) -> VdPipelineResult<()> {
        for instance in self.instance_storage.all_instances_mut() {
            instance.run(self.db)?;
        }
        Ok(())
    }

    pub fn run_all_multi_threaded(&mut self) -> VdPipelineResult<()> {
        use rayon::prelude::*;

        self.instance_storage
            .all_instances_mut()
            .par_iter_mut()
            .try_for_each(|instance| instance.run(self.db))?;
        Ok(())
    }
}
