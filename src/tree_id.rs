use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeId(u32);

const AUTO_ID: u32 = 1 << (std::mem::size_of::<u32>() * 8 - 1);

impl TreeId {
    pub fn new_auto() -> TreeId {
        static NEXT_ID: AtomicU32 = AtomicU32::new(AUTO_ID);

        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert!(id != u32::max_value(), "auto TreeId overflow");

        TreeId(id)
    }

    pub fn new(id: u32) -> TreeId {
        assert!(id < AUTO_ID, "TreeId overflow");
        TreeId(id)
    }
}
