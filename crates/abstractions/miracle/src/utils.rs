use crate::*;

pub(crate) fn with_miracle<T: HasMiracle, R>(
    t: &mut T,
    config: MiracleConfig,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    assert!(
        t.miracle().is_uninitialized(),
        "miracle is already initialized: {:?}",
        t.miracle().inner
    );
    *t.miracle_mut() = Miracle {
        inner: MiracleInner::Initialized {
            state: MiracleState::new(),
            config,
        },
    };
    let r = f(t);
    *t.miracle_mut() = Miracle::new_uninitialized();
    r
}
