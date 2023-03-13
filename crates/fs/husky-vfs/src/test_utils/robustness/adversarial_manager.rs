use super::*;

pub(super) struct VfsAdversarialManager {
    module: ModulePath,
    path: PathBuf,
    adversarials: Vec<VfsAdversarial>,
    generator: Option<VfsAdversarialGenerator>,
}

impl VfsAdversarialManager {
    pub(super) fn new(module: ModulePath, path: PathBuf) -> Self {
        Self {
            module,
            path,
            // todo
            adversarials: vec![],
            generator: VfsAdversarialGenerator::from_env(),
        }
    }

    pub(super) fn run<Db>(mut self, db: &mut Db, f: &(impl Fn(&Db)))
    where
        Db: VfsDb + ?Sized,
    {
        for adversarial in &self.adversarials {
            f(db)
        }
        if let Some(generator) = self.generator {
            match generator.run(db, self.module, f) {
                Ok(_) => (),
                Err(adversarial) => {
                    self.adversarials.push(adversarial);
                    panic!("found an adversarial")
                }
            }
        }
    }
}
