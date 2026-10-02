use std::sync::{Arc, Mutex};

pub struct Context {
    // PlaceHolder
    pub placeholder: u64,
}

pub type SharedMutContext = Arc<Mutex<Context>>;