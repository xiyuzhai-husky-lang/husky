//! Signals - The building blocks of reactivity.

mod read_signal;

use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{AddAssign, Deref, DerefMut, DivAssign, MulAssign, SubAssign};

use husky_signal::Signalable;
pub use read_signal::*;

use crate::effect::EFFECTS;
use crate::*;

type WeakEffectCallback = Weak<RefCell<dyn FnMut()>>;
type EffectCallbackPtr = *const RefCell<dyn FnMut()>;

pub(crate) type SignalEmitterInner = RefCell<IndexMap<EffectCallbackPtr, WeakEffectCallback>>;

/// A struct for managing subscriptions to signals.
#[derive(Default, Clone)]
pub struct SignalEmitter(pub(crate) Rc<SignalEmitterInner>);

#[derive(Default, Clone)]
pub(crate) struct WeakSignalEmitter(pub Weak<SignalEmitterInner>);

impl WeakSignalEmitter {
    pub fn upgrade(&self) -> Option<SignalEmitter> {
        self.0.upgrade().map(SignalEmitter)
    }
}

impl SignalEmitter {
    pub(crate) fn downgrade(&self) -> WeakSignalEmitter {
        WeakSignalEmitter(Rc::downgrade(&self.0))
    }

    /// Adds a callback to the subscriber list. If the callback is already a subscriber, does
    /// nothing.
    pub(crate) fn subscribe(&self, cb: WeakEffectCallback) {
        self.0.borrow_mut().insert(cb.as_ptr(), cb);
    }

    /// Removes a callback from the subscriber list. If the callback is not a subscriber, does
    /// nothing.
    pub(crate) fn unsubscribe(&self, cb: EffectCallbackPtr) {
        self.0.borrow_mut().remove(&cb);
    }

    /// Track the current signal in the effect visibility.
    pub fn track(&self) {
        EFFECTS.with(|effects| {
            if let Some(last) = effects.borrow().last() {
                // SAFETY: See guarantee on EffectState within EFFECTS.
                let last = unsafe { &mut **last };
                last.add_dependency(self.downgrade());
            }
        });
    }

    /// Calls all the subscribers without modifying the state.
    /// This can be useful when using patterns such as inner mutability where the state updated will
    /// not be automatically triggered. In the general case, however, it is preferable to use
    /// [`Signal::set()`] instead.
    ///
    /// This will also re-compute all the subscribers of this signal by calling all the dependency
    /// callbacks.
    pub fn trigger_subscribers(&self) {
        // Reset subscribers to prevent modifying the subscriber list while it is being read from.
        // We can completely wipe out the subscriber list because it will be constructed again when
        // each callback is called.
        let subscribers = self.0.take().into_values();
        // Subscriber order is reversed because effects attach subscribers at the end of the
        // effect visibility. This will ensure that outer effects re-execute before inner effects,
        // preventing inner effects from running twice.
        for subscriber in subscribers.rev() {
            // subscriber might have already been destroyed in the case of nested effects.
            if let Some(callback) = subscriber.upgrade() {
                // Call the callback.
                callback.borrow_mut()()
            }
        }
    }
}

impl<T> Signalable for Signal<T> where T: Signalable {}
impl<T> Signalable for RcSignal<T> where T: Signalable {}
impl<'a> Signalable for ScopeDisposer<'a> {}

#[derive(Default)]
/// Reactive state that can be updated and subscribed to.
pub struct Signal<T>(ReadSignal<T>)
where
    T: Signalable;

impl<T> Signal<T>
where
    T: Signalable,
{
    /// Create a new [`Signal`] with the specified value.
    pub fn new(value: T) -> Self {
        Self(ReadSignal {
            value: RefCell::new(Rc::new(value)),
            emitter: Default::default(),
        })
    }
    pub fn set(&self, value: T) {
        self.set_silent(value);
        self.0.emitter.trigger_subscribers();
    }

    pub fn set_rc(&self, value: Rc<T>) {
        self.set_rc_silent(value);
        self.0.emitter.trigger_subscribers();
    }
    pub fn set_silent(&self, value: T) {
        self.set_rc_silent(Rc::new(value));
    }
    pub fn set_rc_silent(&self, value: Rc<T>) {
        *self.0.value.borrow_mut() = value;
    }
    pub fn split(&self) -> (impl Fn() -> Rc<T> + Copy + '_, impl Fn(T) + Copy + '_) {
        let getter = move || self.get();
        let setter = move |x| self.set(x);
        (getter, setter)
    }
    pub fn read(&self) -> &ReadSignal<T> {
        &self.0
    }
}

/// A mutable reference for modifying a [`Signal`].
///
/// Construct this using the [`Signal::modify()`] method.
pub struct Modify<'a, T>(Option<T>, &'a Signal<T>)
where
    T: Signalable;

impl<'a, T> Deref for Modify<'a, T>
where
    T: Signalable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
impl<'a, T> DerefMut for Modify<'a, T>
where
    T: Signalable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

/// When the mutable handle is dropped, update the [`Signal`].
impl<T> Drop for Modify<'_, T>
where
    T: Signalable,
{
    fn drop(&mut self) {
        self.1.set(self.0.take().unwrap())
    }
}

impl<T: Clone> Signal<T>
where
    T: Signalable,
{
    pub fn modify(&self) -> Modify<T> {
        Modify(Some(self.value.borrow().as_ref().clone()), self)
    }
}

impl<T: Default> Signal<T>
where
    T: Signalable,
{
    pub fn take(&self) -> Rc<T> {
        let ret = self.0.value.take();
        self.0.emitter.trigger_subscribers();
        ret
    }

    pub fn take_silent(&self) -> Rc<T> {
        self.0.value.take()
    }
}

impl<T> Deref for Signal<T>
where
    T: Signalable,
{
    type Target = ReadSignal<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AddAssign + Copy> AddAssign<T> for &Signal<T>
where
    T: Signalable,
{
    fn add_assign(&mut self, other: T) {
        let mut value = **self.0.value.borrow();
        value += other;
        self.set(value);
    }
}
impl<T: SubAssign + Copy> SubAssign<T> for &Signal<T>
where
    T: Signalable,
{
    fn sub_assign(&mut self, other: T) {
        let mut value = **self.0.value.borrow();
        value -= other;
        self.set(value);
    }
}
impl<T: MulAssign + Copy> MulAssign<T> for &Signal<T>
where
    T: Signalable,
{
    fn mul_assign(&mut self, other: T) {
        let mut value = **self.0.value.borrow();
        value *= other;
        self.set(value);
    }
}
impl<T: DivAssign + Copy> DivAssign<T> for &Signal<T>
where
    T: Signalable,
{
    fn div_assign(&mut self, other: T) {
        let mut value = **self.0.value.borrow();
        value /= other;
        self.set(value);
    }
}

/// A trait that is implemented for all [`ReadSignal`]s regardless of the type parameter.
pub trait AnyReadSignal<'a> {
    /// Call the [`ReadSignal::track`] method.
    fn track(&self);
}
impl<'a, T> AnyReadSignal<'a> for RcSignal<T>
where
    T: Signalable,
{
    fn track(&self) {
        self.deref().deref().track();
    }
}
impl<'a, T> AnyReadSignal<'a> for Signal<T>
where
    T: Signalable,
{
    fn track(&self) {
        self.deref().track();
    }
}
impl<'a, T> AnyReadSignal<'a> for ReadSignal<T> {
    fn track(&self) {
        self.track();
    }
}
pub fn create_signal<T>(visibility: Scope, value: T) -> &Signal<T>
where
    T: Signalable,
{
    let signal = Signal::new(value);
    create_ref(visibility, signal)
}

pub fn create_signal_from_rc<T>(visibility: Scope, value: Rc<T>) -> &Signal<T>
where
    T: Signalable,
{
    let signal = Signal(ReadSignal {
        value: RefCell::new(value),
        emitter: Default::default(),
    });
    create_ref(visibility, signal)
}

pub struct RcSignal<T>(Rc<Signal<T>>)
where
    T: Signalable;

impl<T> Deref for RcSignal<T>
where
    T: Signalable,
{
    type Target = Signal<T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T> Clone for RcSignal<T>
where
    T: Signalable,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub fn create_rc_signal<T>(value: T) -> RcSignal<T>
where
    T: Signalable,
{
    RcSignal(Rc::new(Signal::new(value)))
}

pub fn create_rc_signal_from_rc<T>(value: Rc<T>) -> RcSignal<T>
where
    T: Signalable,
{
    RcSignal(Rc::new(Signal(ReadSignal {
        value: RefCell::new(value),
        emitter: Default::default(),
    })))
}

impl<T: Display> Display for RcSignal<T>
where
    T: Signalable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.get(), f)
    }
}
impl<T: Display> Display for Signal<T>
where
    T: Signalable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.get(), f)
    }
}
impl<T: Display> Display for ReadSignal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Debug> Debug for RcSignal<T>
where
    T: Signalable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RcSignal").field(&self.get()).finish()
    }
}
impl<T: Debug> Debug for Signal<T>
where
    T: Signalable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Signal").field(&self.get()).finish()
    }
}
impl<T: Debug> Debug for ReadSignal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadSignal").field(&self.get()).finish()
    }
}

/* Default implementations */

impl<T: Default> Default for RcSignal<T>
where
    T: Signalable,
{
    fn default() -> Self {
        create_rc_signal(T::default())
    }
}

/* PartialEq, Eq, Hash implementations */

impl<T: PartialEq> PartialEq for RcSignal<T>
where
    T: Signalable,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_untracked().eq(&other.get_untracked())
    }
}
impl<T: PartialEq> PartialEq for Signal<T>
where
    T: Signalable,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_untracked().eq(&other.get_untracked())
    }
}
impl<T: PartialEq> PartialEq for ReadSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_untracked().eq(&other.get_untracked())
    }
}

impl<T: Eq> Eq for RcSignal<T> where T: Signalable {}
impl<T: Eq> Eq for Signal<T> where T: Signalable {}
impl<T: Eq> Eq for ReadSignal<T> where T: Signalable {}

impl<T: Hash> Hash for RcSignal<T>
where
    T: Signalable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_untracked().hash(state)
    }
}
impl<T: Hash> Hash for Signal<T>
where
    T: Signalable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_untracked().hash(state)
    }
}
impl<T: Hash> Hash for ReadSignal<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_untracked().hash(state)
    }
}

/* Serde implementations */

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for RcSignal<T>
where
    T: Signalable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get().serialize(serializer)
    }
}
#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for RcSignal<T>
where
    T: Signalable,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(create_rc_signal(T::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            assert_eq!(*state.get(), 0);

            state.set(1);
            assert_eq!(*state.get(), 1);
        });
    }

    #[test]
    fn signal_composition() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = || *state.get() * 2;

            assert_eq!(double(), 0);
            state.set(1);
            assert_eq!(double(), 2);
        });
    }

    #[test]
    fn set_silent_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = state.map(cx, |&x| x * 2);

            assert_eq!(*double.get(), 0);
            state.set_silent(1);
            assert_eq!(*double.get(), 0); // double value is unchanged.
        });
    }

    #[test]
    fn read_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let readonly: &ReadSignal<i32> = state.deref();

            assert_eq!(*readonly.get(), 0);
            state.set(1);
            assert_eq!(*readonly.get(), 1);
        });
    }

    #[test]
    fn map_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 0);
            let double = state.map(cx, |&x| x * 2);

            assert_eq!(*double.get(), 0);
            state.set(1);
            assert_eq!(*double.get(), 2);
        });
    }

    #[test]
    fn take_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 123);

            let x = state.take();
            assert_eq!(*x, 123);
            assert_eq!(*state.get(), 0);
        });
    }

    #[test]
    fn take_silent_signal() {
        create_scope_immediate(|cx| {
            let state = create_signal(cx, 123);
            let double = state.map(cx, |&x| x * 2);

            // Do not trigger subscribers.
            state.take_silent();
            assert_eq!(*state.get(), 0);
            assert_eq!(*double.get(), 246);
        });
    }

    #[test]
    fn signal_split() {
        create_scope_immediate(|cx| {
            let (state, set_state) = create_signal(cx, 0).split();
            assert_eq!(*state(), 0);

            set_state(1);
            assert_eq!(*state(), 1);
        });
    }

    #[test]
    fn rc_signal() {
        let mut outer = None;
        create_scope_immediate(|cx| {
            let rc_state = create_rc_signal(0);
            let rc_state_cloned = rc_state.clone();
            let double = create_memo(
                cx,
                move || *rc_state_cloned.get() * 2,
                format!("src at {}:{}", file!(), line!()),
            );
            assert_eq!(*double.get(), 0);

            rc_state.set(1);
            assert_eq!(*double.get(), 2);

            outer = Some(rc_state);
        });
        assert_eq!(*outer.unwrap().get(), 1);
    }

    #[test]
    fn signal_display() {
        create_scope_immediate(|cx| {
            let signal = create_signal(cx, 0);
            assert_eq!(format!("{signal}"), "0");
            let read_signal: &ReadSignal<_> = signal;
            assert_eq!(format!("{read_signal}"), "0");
            let rcsignal = create_rc_signal(0);
            assert_eq!(format!("{rcsignal}"), "0");
        });
    }

    #[test]
    fn signal_debug() {
        create_scope_immediate(|cx| {
            let signal = create_signal(cx, 0);
            assert_eq!(format!("{signal:?}"), "Signal(0)");
            let read_signal: &ReadSignal<_> = signal;
            assert_eq!(format!("{read_signal:?}"), "ReadSignal(0)");
            let rcsignal = create_rc_signal(0);
            assert_eq!(format!("{rcsignal:?}"), "RcSignal(0)");
        });
    }

    #[test]
    fn signal_add_assign_update() {
        create_scope_immediate(|cx| {
            let mut signal = create_signal(cx, 0);
            let counter = create_signal(cx, 0);
            effect!(cx, || {
                signal.track();
                counter.set(*counter.get_untracked() + 1);
            });
            signal += 1;
            signal -= 1;
            signal *= 1;
            signal /= 1;
            assert_eq!(*counter.get(), 5);
        });
    }

    #[test]
    fn signal_modify() {
        create_scope_immediate(|cx| {
            let signal = create_signal(cx, "Hello ".to_string());
            let counter = create_signal(cx, 0);
            effect!(cx, || {
                signal.track();
                counter.set(*counter.get_untracked() + 1);
            });
            signal.modify().push_str("World!");
            assert_eq!(*signal.get(), "Hello World!");
            assert_eq!(*counter.get(), 2);
        });
    }

    #[test]
    fn create_signals_from_rc_value() {
        create_scope_immediate(|cx| {
            let _signal: &Signal<i32> = create_signal_from_rc(cx, Rc::new(0));
            let _rc_signal: RcSignal<i32> = create_rc_signal_from_rc(Rc::new(0));
        });
    }
}
