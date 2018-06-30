use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(new)]
pub struct Revision {
    counter: AtomicUsize,
}

const REVISION: AtomicUsize = AtomicUsize::new(0);

pub trait Tag {
    fn revision(&self) -> usize;
    fn is_valid(&self, snapshot: usize) -> bool;
}

pub struct DirtyableTag {
    revision: AtomicUsize,
}

impl DirtyableTag {
    pub fn new() -> DirtyableTag {
        DirtyableTag {
            revision: AtomicUsize::new(REVISION.load(Ordering::SeqCst)),
        }
    }

    pub fn dirty(&self) {
        let last_value = REVISION.fetch_add(1, Ordering::SeqCst);
        self.revision.store(last_value + 1, Ordering::SeqCst);
    }
}

impl Tag for DirtyableTag {
    fn revision(&self) -> usize {
        self.revision.load(Ordering::SeqCst)
    }

    fn is_valid(&self, snapshot: usize) -> bool {
        self.revision.load(Ordering::SeqCst) == snapshot
    }
}
