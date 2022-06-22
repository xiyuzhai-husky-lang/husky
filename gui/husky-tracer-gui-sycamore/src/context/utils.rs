use super::*;

impl DebuggerContext {
    pub fn create_static_ref<T>(&self, data: T) -> &'static T {
        create_static_ref(self.scope, data)
    }
}
