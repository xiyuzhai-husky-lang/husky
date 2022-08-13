use super::*;

impl DebuggerContext {
    pub fn alloc_value<T>(&self, value: T) -> &'static T {
        create_static_ref(self.scope, value)
    }
    pub fn alloc_signal<T>(&self, value: T) -> &'static Signal<T>
    where
        T: Signalable,
    {
        create_static_signal(self.scope, value)
    }

    pub(super) fn alloc_key_value_pairs<K, V>(
        &self,
        data: Vec<(K, V)>,
    ) -> impl Iterator<Item = (K, &'static V)> + '_
    where
        K: 'static,
        V: 'static,
    {
        data.into_iter().map(|(k, v)| (k, self.alloc_value(v)))
    }

    pub(super) fn alloc_key_signal_pairs<K, V>(
        &self,
        data: Vec<(K, V)>,
    ) -> impl Iterator<Item = (K, &'static Signal<V>)> + '_
    where
        K: 'static,
        V: 'static + Signalable,
    {
        data.into_iter().map(|(k, v)| (k, self.alloc_signal(v)))
    }
}
