//! Side effects.

use ahash::AHashSet;

use crate::*;

thread_local! {
    /// While the [`EffectState`] is inside the Vec, it is owned by [`EFFECTS`].
    /// Because this is a global variable, the lifetime is necessarily `'static`. However, that does not mean
    /// that it can last forever. The `EffectState` should only be used the time it is inside [`EFFECTS`].
    pub(crate) static EFFECTS: RefCell<Vec<*mut EffectState< 'static>>>  = Default::default();
}

/// The internal state of an effect. The effect callback and the effect dependencies are stored in
/// this struct.
pub(crate) struct EffectState<'a> {
    /// The callback when the effect is re-executed.
    cb: Rc<RefCell<dyn FnMut() + 'a>>,
    /// A list of dependencies that can trigger this effect.
    dependencies: AHashSet<EffectDependency>,
    pub(crate) info: String,
}

/// Implements reference equality for [`WeakSignalEmitter`]s.
pub(crate) struct EffectDependency(WeakSignalEmitter);
impl std::cmp::PartialEq for EffectDependency {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0 .0) == Weak::as_ptr(&other.0 .0)
    }
}
impl std::cmp::Eq for EffectDependency {}
impl std::hash::Hash for EffectDependency {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Weak::as_ptr(&self.0 .0).hash(state);
    }
}

impl<'a> EffectState<'a> {
    // Clears the dependencies (both links and backlinks).
    /// Should be called when re-executing an effect to recreate all dependencies.
    pub fn clear_dependencies(&mut self) {
        for dependency in &self.dependencies {
            if let Some(dep) = dependency.0.upgrade() {
                // SAFETY: We only access the pointer, not the pointed data.
                dep.unsubscribe(unsafe { std::mem::transmute(Rc::as_ptr(&self.cb)) })
            };
        }
        self.dependencies.clear();
    }

    /// Add a dependency to the effect.
    pub fn add_dependency(&mut self, signal: WeakSignalEmitter) {
        self.dependencies.insert(EffectDependency(signal));
    }
}

/// Creates an effect on signals used inside the effect closure.
///
/// # Example
/// ```
/// # use sycamore_reactive::*;
/// # create_scope_immediate(|cx| {
/// let state = create_signal(cx, 0);
///
/// effect!(cx, || {
///     println!("State changed. New state value = {}", state.get());
/// }); // Prints "State changed. New state value = 0"
///
/// state.set(1); // Prints "State changed. New state value = 1"
/// # });
/// ```
pub fn create_effect<'a>(cx: Scope<'a>, f: impl FnMut() + 'a, info: String) {
    let f = cx.alloc(f);
    _create_effect(cx, f, info)
}

#[macro_export]
macro_rules! effect {
    ($visibility: ident, $f: expr) => {{
        create_effect(
            $visibility,
            $f,
            format!("effect at {}:{}", file!(), line!()),
        )
    }};
}

/// Internal implementation for `create_effect`. Use dynamic dispatch to reduce code-bloat.
fn _create_effect<'a>(cx: Scope<'a>, f: &'a mut (dyn FnMut() + 'a), info: String) {
    let effect = &*cx.alloc(RefCell::new(None::<EffectState<'a>>));
    let cb = Rc::new(RefCell::new({
        move || {
            EFFECTS.with(|effects| {
                // Record initial effect stack length to verify that it is the same after.
                let initial_effect_stack_len = effects.borrow().len();

                // Take effect out.
                let mut tmp_effect = effect.take().unwrap();
                tmp_effect.clear_dependencies();

                // Push the effect onto the effect stack so that it is visible by signals.
                effects
                    .borrow_mut()
                    .push((&mut tmp_effect as *mut EffectState<'a>).cast::<EffectState<'static>>());
                // Now we can call the user-provided function.
                f();
                // Pop the effect from the effect stack.
                effects.borrow_mut().pop().unwrap();
                // The raw pointer pushed onto `effects` is dead and can no longer be accessed.
                // We can now access `effect` directly again.

                // For all the signals collected by the EffectState, we need to add backlinks from
                // the signal to the effect, so that updating the signal will trigger the effect.
                for emitter in &tmp_effect.dependencies {
                    // The SignalEmitter might have been destroyed between when the signal was
                    // accessed and now.
                    if let Some(emitter) = emitter.0.upgrade() {
                        // SAFETY: When the effect is destroyed or when the emitter is dropped,
                        // this link will be destroyed to prevent dangling references.
                        emitter.subscribe(Rc::downgrade(unsafe {
                            std::mem::transmute(&tmp_effect.cb)
                        }));
                    }
                }

                // Get the effect state back into the Rc
                match effect.try_borrow_mut() {
                    Ok(mut effect) => *effect = Some(tmp_effect),
                    Err(_) => {
                        log::info!("tmp_effect info = {}", tmp_effect.info);
                        todo!()
                    }
                }

                debug_assert_eq!(effects.borrow().len(), initial_effect_stack_len);
            });
        }
    }));

    // Initialize initial effect state.
    *effect.borrow_mut() = Some(EffectState {
        cb: cb.clone(),
        dependencies: AHashSet::new(),
        info,
    });

    // Initial callback call to get everything started.
    cb.borrow_mut()();
}

pub fn create_effect_scoped<'a, F>(cx: Scope<'a>, mut f: F, info: String)
where
    F: for<'child_lifetime> FnMut(BoundedScope<'child_lifetime, 'a>) + 'a,
{
    let mut disposer: Option<ScopeDisposer<'a>> = None;
    create_effect(
        cx,
        move || {
            // We run the disposer inside the effect, after effect dependencies have been cleared.
            // This is to make sure that if the effect subscribes to its own signal, there is no
            // use-after-free during the clear dependencies phase.
            if let Some(disposer) = disposer.take() {
                // SAFETY: we are not accessing the visibility after the effect has been dropped.
                unsafe { disposer.dispose() };
            }
            // Create a new nested visibility and save the disposer.
            let new_disposer: Option<ScopeDisposer<'a>> = Some(create_child_scope(cx, |cx| {
                // SAFETY: f takes the same parameter as the argument to
                // self.create_child_scope(_).
                f(unsafe { std::mem::transmute(cx) });
            }));
            disposer = new_disposer;
        },
        info,
    );
}

pub fn untrack<T>(f: impl FnOnce() -> T) -> T {
    EFFECTS.with(|effects| {
        let tmp = effects.take();
        let ret = f();
        *effects.borrow_mut() = tmp;
        ret
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);

            let double = create_signal(cx, -1);

            effect!(cx, || {
                double.set(*state.get() * 2);
            });
            assert_eq!(*double.get(), 0); // calling create_effect should call the effect at least once

            state.set(1);
            assert_eq!(*double.get(), 2);
            state.set(2);
            assert_eq!(*double.get(), 4);
        });
    }

    #[test]
    fn effect_with_explicit_dependencies() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);

            let double = create_signal(cx, -1);

            effect!(
                cx,
                on([state], || {
                    double.set(*state.get() * 2);
                })
            );
            assert_eq!(*double.get(), 0); // calling create_effect should call the effect at least once

            state.set(1);
            assert_eq!(*double.get(), 2);
            state.set(2);
            assert_eq!(*double.get(), 4);
        });
    }

    #[test]
    fn effect_cannot_create_infinite_loop() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            effect!(cx, || {
                state.track();
                state.set(0);
            });
            state.set(0);
        });
    }

    #[test]
    fn effect_should_only_subscribe_once_to_same_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);

            let counter = create_signal(cx, 0);
            effect!(cx, || {
                counter.set(*counter.get_untracked() + 1);

                // call state.track() twice but should subscribe once
                state.track();
                state.track();
            });

            assert_eq!(*counter.get(), 1);

            state.set(1);
            assert_eq!(*counter.get(), 2);
        });
    }

    #[test]
    fn effect_should_recreate_dependencies_each_time() {
        create_scope_immediate(|cx| {
            let condition = create_signal(cx, true);

            let state1 = create_signal(cx, 0);
            let state2 = create_signal(cx, 1);

            let counter = create_signal(cx, 0);
            effect!(cx, || {
                counter.set(*counter.get_untracked() + 1);

                if *condition.get() {
                    state1.track();
                } else {
                    state2.track();
                }
            });

            assert_eq!(*counter.get(), 1);

            state1.set(1);
            assert_eq!(*counter.get(), 2);

            state2.set(1);
            assert_eq!(*counter.get(), 2); // not tracked

            condition.set(false);
            assert_eq!(*counter.get(), 3);

            state1.set(2);
            assert_eq!(*counter.get(), 3); // not tracked

            state2.set(2);
            assert_eq!(*counter.get(), 4); // tracked after condition.set
        });
    }

    #[test]
    fn outer_effects_run_first() {
        create_scope_immediate(|cx| {
            let trigger = create_signal(cx, ());

            let outer_counter = create_signal(cx, 0);
            let inner_counter = create_signal(cx, 0);

            create_effect_scoped(
                cx,
                |cx| {
                    trigger.track();
                    outer_counter.set(*outer_counter.get_untracked() + 1);

                    effect!(cx, || {
                        trigger.track();
                        inner_counter.set(*inner_counter.get_untracked() + 1);
                    });
                },
                "haha".into(),
            );

            assert_eq!(*outer_counter.get(), 1);
            assert_eq!(*inner_counter.get(), 1);

            trigger.set(());

            assert_eq!(*outer_counter.get(), 2);
            assert_eq!(*inner_counter.get(), 2);
        });
    }

    #[test]
    fn destroy_effects_on_scope_dispose() {
        create_scope_immediate(|cx| {
            let counter = create_signal(cx, 0);

            let trigger = create_signal(cx, ());

            let disposer = create_child_scope(cx, |cx| {
                effect!(cx, || {
                    trigger.track();
                    counter.set(*counter.get_untracked() + 1);
                });
            });

            assert_eq!(*counter.get(), 1);

            trigger.set(());
            assert_eq!(*counter.get(), 2);

            unsafe {
                disposer.dispose();
            }
            trigger.set(());
            assert_eq!(*counter.get(), 2); // inner effect should be destroyed and thus not executed
        });
    }

    #[test]
    fn effect_preserves_scope_hierarchy() {
        create_scope_immediate(|cx| {
            let trigger = create_signal(cx, ());
            let parent: &Signal<Option<*const c_void>> = create_signal(cx, None);
            create_effect_scoped(
                cx,
                |cx| {
                    trigger.track();
                    let p = cx.raw.parent.unwrap();
                    parent.set(Some(p as *const c_void));
                },
                "haha".into(),
            );
            assert_eq!(
                parent.get().unwrap(),
                cx.raw as *const _ as *const c_void,
                "the parent visibility of the effect should be `cx`"
            );
            trigger.set(());
            assert_eq!(
                parent.get().unwrap(),
                cx.raw as *const _ as *const c_void,
                "the parent should still be `cx` after effect is re-executed"
            );
        });
    }

    #[test]
    fn effect_scoped_subscribing_to_own_signal() {
        create_scope_immediate(|cx| {
            let trigger = create_signal(cx, ());
            create_effect_scoped(
                cx,
                |cx| {
                    trigger.track();
                    let signal = create_signal(cx, ());
                    // Track own signal:
                    signal.track();
                },
                format!("src at {}:{}", file!(), line!()),
            );
            trigger.set(());
        });
    }

    #[test]
    fn effect_do_not_subscribe_to_destroyed_signal() {
        create_scope_immediate(|cx| {
            let trigger = create_signal(cx, ());
            let mut signal = Some(create_rc_signal(()));
            effect!(cx, move || {
                trigger.track();
                if let Some(signal) = signal.take() {
                    signal.track();
                    drop(signal);
                }
            });
            trigger.set(());
        });
    }
}
