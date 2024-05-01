use std::sync::atomic::{AtomicI32, Ordering};

pub struct BoundAtomic {
    min: i32,
    max: i32,
    integer: AtomicI32,
}

impl BoundAtomic {
    pub fn new(min: i32, max: i32) -> Self {
        assert!(min <= max, "min must be less than or equal to max");
        Self {
            min,
            max,
            integer: AtomicI32::new(min),
        }
    }

    pub fn next_val(&self) -> i32 {
        let next = self.integer.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
            Some(if x >= self.max { self.min } else { x + 1 })
        }).expect("fetch_update should not fail");
        next
    }
}
