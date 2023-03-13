use std::panic::AssertUnwindSafe;

use super::*;

pub(super) struct VfsAdversarialGenerator {
    round: u16,
}

impl VfsAdversarialGenerator {
    pub(super) fn from_env() -> Option<Self> {
        let Ok(robustness_test) = std::env::var("ROBUSTNESS_TEST") else {
            return None
        };
        let round: u16 = robustness_test.parse().expect("valid u8");
        assert!(round < 1000);
        Some(Self { round })
    }

    pub(super) fn run<Db>(
        self,
        db: &mut Db,
        module: ModulePath,
        f: &(impl Fn(&Db)),
    ) -> Result<(), VfsAdversarial>
    where
        Db: VfsDb + ?Sized,
    {
        for _ in 0..self.round {
            self.run_step(db, module, f)?
        }
        Ok(())
    }

    fn run_step<Db>(
        &self,
        db: &mut Db,
        module: ModulePath,
        f: &(impl Fn(&Db)),
    ) -> Result<(), VfsAdversarial>
    where
        Db: VfsDb + ?Sized,
    {
        let adversarial = self.generate_adversarial();
        match std::panic::catch_unwind(AssertUnwindSafe(|| f(db))) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    fn generate_adversarial(&self) -> VfsAdversarial {
        todo!()
    }
}
