use std::sync::{Arc, RwLock};

/// A clonable unique handle; no two `Unique`s created with `Unique::new` can be equal, but a `Unique` and its clone will always be equal
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unique {
    id: Arc<u64>,
}

impl Unique {
    /// Create a new `Unique`
    pub fn new() -> Self {
        let mut next_id = NEXT_ID.write().unwrap();
        let id = *next_id + 1;
        *next_id = id;
        Unique { id: Arc::new(id) }
    }
}

lazy_static::lazy_static! {
    static ref NEXT_ID: Arc<RwLock<u64>> = Arc::new(RwLock::new(0));
}
