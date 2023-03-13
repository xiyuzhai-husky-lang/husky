use salsa::DbWithJar;
use std::panic::AssertUnwindSafe;
use xrng::XRng;

use super::*;

pub(super) struct VfsAdversarialGenerator {
    round: u32,
    rng: XRng,
}

impl VfsAdversarialGenerator {
    pub(super) fn from_env() -> Option<Self> {
        let Ok(round) = std::env::var("ADVERSARIAL_ROUND") else {
            return None
        };
        let round: u32 = round.parse().expect("valid u32");
        Some(Self {
            round,
            rng: XRng::new_time_seeded(),
        })
    }

    pub(super) fn run<Db>(
        mut self,
        db: &mut Db,
        module_path: ModulePath,
        f: &(impl Fn(&Db)),
    ) -> Result<(), VfsAdversarial>
    where
        Db: VfsDb + ?Sized,
    {
        use indicatif::ProgressBar;

        let bar = ProgressBar::new(self.round as u64);
        for i in 0..self.round {
            bar.inc(1);
            self.run_step(db, module_path, f)?
        }
        Ok(())
    }

    fn run_step<Db>(
        &mut self,
        db: &mut Db,
        module_path: ModulePath,
        f: &(impl Fn(&Db)),
    ) -> Result<(), VfsAdversarial>
    where
        Db: VfsDb + ?Sized,
    {
        let adversarial = self.generate_adversarial(db.module_content(module_path).unwrap());
        match adversarial.test(db, module_path, f) {
            Ok(_) => Ok(()),
            Err(_) => Err(adversarial),
        }
    }

    fn generate_adversarial(&mut self, text: &str) -> VfsAdversarial {
        VfsAdversarial::new_rand(text, &mut self.rng)
    }
}
