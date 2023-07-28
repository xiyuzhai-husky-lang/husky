use super::*;

impl DeveloperGuiContext {
    pub fn alloc_value<T>(&self, value: T) -> &'static T {
        create_leash(self.visibility, value)
    }
    pub fn alloc_signal<T>(&self, value: T) -> &'static Signal<T>
    where
        T: Signalable,
    {
        create_static_signal(self.visibility, value)
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

    pub(super) fn alloc_key_opt_value_pairs<K, V>(
        &self,
        data: Vec<(K, Option<V>)>,
    ) -> impl Iterator<Item = (K, Option<&'static V>)> + '_
    where
        K: 'static,
        V: 'static,
    {
        data.into_iter()
            .map(|(k, v)| (k, v.map(|v| self.alloc_value(v))))
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
