use std::sync::Arc;

pub type Avec<T> = Arc<Vec<Arc<T>>>;
