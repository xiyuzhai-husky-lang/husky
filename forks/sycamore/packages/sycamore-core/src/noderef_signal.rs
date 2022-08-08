//! References to nodes in views.

use std::any::Any;
use std::fmt;
use std::rc::Rc;

use husky_signal::Signalable;
use sycamore_reactive::*;

use crate::generic_node::GenericNode;

// this doc test doesn't work properly

/// A reference to a [`GenericNode`].
/// This allows programmatically accessing the node and call imperative methods on it.
///
/// # Example
/// ```no run
/// use sycamore::prelude::*;
/// use sycamore_core::view::View;
///
/// #[component]
/// fn Component<G: Html>(cx: Scope) -> View<G> {
///     use sycamore_core::noderef_signal::create_node_ref_signal;
///     let my_div = create_node_ref_signal(cx);
///     view! { cx,
///         div(ref=my_div)
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NodeRefSignal<G: GenericNode>(pub Rc<Signal<Option<G>>>)
where
    G: Signalable;

impl<G: GenericNode + Any> NodeRefSignal<G>
where
    G: Signalable,
{
    /// Creates an empty [`NodeRefSignal`].
    ///
    /// Generally, it is preferable to use [`create_node_ref`]
    /// instead.
    pub fn new() -> Self {
        Self(Rc::new(Signal::new(None)))
    }

    /// Gets the T stored inside the [`NodeRefSignal`].
    ///
    /// # Panics
    /// Panics if the [`NodeRefSignal`] is not set yet or is the wrong type.
    ///
    /// For a non panicking version, see [`NodeRefSignal::try_get`].
    #[track_caller]
    pub fn get<T: GenericNode>(&self) -> T {
        self.try_get().expect("NodeRefSignal is not set")
    }

    /// Tries to get the T stored inside the [`NodeRefSignal`] or `None` if it is not yet set or
    /// the wrong type.
    ///
    /// For a panicking version, see [`NodeRefSignal::get`].
    pub fn try_get<T: GenericNode>(&self) -> Option<T> {
        let obj = self.0.get();
        ((*obj).as_ref()? as &dyn Any).downcast_ref().cloned()
    }

    /// Gets the raw [`GenericNode`] stored inside the [`NodeRefSignal`].
    ///
    /// # Panics
    /// Panics if the [`NodeRefSignal`] is not set yet.
    ///
    /// For a non panicking version, see [`NodeRefSignal::try_get_raw`].
    #[track_caller]
    pub fn get_raw(&self) -> G {
        self.try_get().expect("NodeRefSignal is not set")
    }

    /// Tries to get the raw [`GenericNode`] stored inside the [`NodeRefSignal`] or `None` if it is
    /// not yet set.
    ///
    /// For a panicking version, see [`NodeRefSignal::get`].
    pub fn try_get_raw(&self) -> Option<G> {
        self.0.cget()
    }

    /// Sets the [`NodeRefSignal`] with the specified [`GenericNode`].
    ///
    /// This method should be rarely used. Instead, use the `ref=` syntax in the `view!` macro to
    /// set the node.
    pub fn set(&self, node: G) {
        self.0.set(Some(node))
    }
}

impl<G: GenericNode> Default for NodeRefSignal<G>
where
    G: Signalable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<G: GenericNode> fmt::Debug for NodeRefSignal<G>
where
    G: Signalable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeRefSignal").field(&self.0.get()).finish()
    }
}

/* Hook implementation */

/// Create a new [`NodeRefSignal`] on the current [`Scope`].
pub fn create_node_ref_signal<G: GenericNode>(cx: Scope<'_>) -> &NodeRefSignal<G>
where
    G: Signalable,
{
    create_ref(cx, NodeRefSignal::new())
}
