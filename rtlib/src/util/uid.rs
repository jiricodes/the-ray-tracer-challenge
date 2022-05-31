use std::sync::atomic::{AtomicUsize, Ordering};

static UID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn fetch_uid() -> usize {
    UID_COUNTER.fetch_add(1, Ordering::SeqCst)
}
