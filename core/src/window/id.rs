use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{self, AtomicU64};

/// The id of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(u64);

/// Id of the "Global" window. Will never match a window Id,
/// which means any message sent to it will automatically be marked as Status::Ignored
pub const GLOBAL: Id = Id(0);
static COUNT: AtomicU64 = AtomicU64::new(1);

impl Id {
    /// Creates a new unique window [`Id`].
    pub fn unique() -> Id {
        Id(COUNT.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
