//! Named after Gerard Valkyrie's "The Miracle" from Bleach, a divine ability that
//! continuously revives its wielder to become stronger after each defeat.

pub mod config;
pub mod error;
pub mod fold;
pub mod metric;
pub mod state;

use self::{config::*, error::*, metric::*, state::*};
use alt_maybe_result::*;
use alt_option::*;
use sealed::sealed;

pub struct Miracle {
    inner: MiracleInner,
}

impl Miracle {
    pub fn is_uninitialized(&self) -> bool {
        matches!(self.inner, MiracleInner::Uninitialized)
    }
}

pub enum MiracleInner {
    Uninitialized,
    Initialized {
        state: MiracleState,
        config: MiracleConfig,
    },
}

impl Miracle {
    pub fn new() -> Self {
        Self {
            inner: MiracleInner::Uninitialized,
        }
    }
}

impl Miracle {
    pub fn state(&self) -> &MiracleState {
        match &self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { state, config } => state,
        }
    }

    pub fn config(&self) -> &MiracleConfig {
        match &self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { config, .. } => config,
        }
    }

    pub fn state_mut(&mut self) -> &mut MiracleState {
        match &mut self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { state, config } => state,
        }
    }
}

pub trait HasMiracle {
    fn miracle(&self) -> &Miracle;
    fn miracle_mut(&mut self) -> &mut Miracle;
}

#[sealed]
pub trait HasMiracleFull: HasMiracle {
    fn run_staged<R>(
        self,
        stages: &[f64],
        max_heartbeats: u64,
        f: impl FnMut(&mut Self) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>;

    fn split<R>(
        &mut self,
        number_of_values: u64,
        f: impl FnMut(&mut Self, u64) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>;
}

#[sealed]
impl<T: HasMiracle> HasMiracleFull for T {
    fn run_staged<R>(
        mut self,
        stages: &[f64],
        max_heartbeats: u64,
        mut f: impl FnMut(&mut Self) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R> {
        assert!(self.miracle().is_uninitialized());
        let fst = *stages.first().unwrap();
        assert!(fst >= 0.0);
        for (norm_low, norm_high) in [(0.0, fst)]
            .into_iter()
            .chain(stages.windows(2).map(|w| (w[0], w[1])))
        {
            *self.miracle_mut() = Miracle {
                inner: MiracleInner::Initialized {
                    state: MiracleState::new(),
                    config: MiracleConfig {
                        norm_low,
                        norm_high,
                        max_heartbeats,
                    },
                },
            };
            f(&mut self)?;
        }
        AltNothing
    }

    fn split<R>(
        &mut self,
        number_of_values: u64,
        mut f: impl FnMut(&mut Self, u64) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R> {
        for i in 0..number_of_values {
            crate::state::calc_with_new_value_appended(self, i, |g| f(g, i))?;
        }
        AltNothing
    }
}

#[test]
fn run_staged_alt_option_works() {
    struct Gerald {
        miracle: Miracle,
    }

    impl HasMiracle for Gerald {
        fn miracle(&self) -> &Miracle {
            &self.miracle
        }
        fn miracle_mut(&mut self) -> &mut Miracle {
            &mut self.miracle
        }
    }

    let mut gerald = Gerald {
        miracle: Miracle::new(),
    };
    assert_eq!(
        gerald.run_staged(&[1.0], 10, |_| AltJustOk(1)),
        AltJustOk(1)
    );
}
