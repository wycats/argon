use crate::prelude::*;

const REVISION: AtomicUsize = AtomicUsize::new(0);

pub fn revision() -> usize {
    REVISION.load(Ordering::SeqCst)
}

pub fn bump() -> usize {
    REVISION.fetch_add(1, Ordering::SeqCst)
}

pub struct VersionedCell<T> {
    revision: usize,
    value: Arcish<T>,
}

impl<T> VersionedCell<T> {
    pub fn new(value: T) -> VersionedCell<T> {
        VersionedCell {
            revision: revision(),
            value: Arcish::strong(value),
        }
    }

    pub fn from(arcish: Arcish<T>) -> VersionedCell<T> {
        VersionedCell {
            revision: revision(),
            value: arcish,
        }
    }

    pub fn take_weakly(arc: &Arc<T>) -> VersionedCell<T> {
        VersionedCell {
            revision: revision(),
            value: Arcish::weak(arc),
        }
    }

    pub fn revision(&self) -> usize {
        self.revision
    }

    pub fn weak(&self) -> VersionedCell<T> {
        VersionedCell {
            revision: self.revision,
            value: self.value.as_weak_arcish(),
        }
    }

    pub fn update(&self, value: T) -> VersionedCell<T> {
        VersionedCell {
            revision: bump(),
            value: Arcish::strong(value),
        }
    }

    pub fn update_with(&self, f: impl FnOnce(Arcish<T>) -> T) -> VersionedCell<T> {
        let value = f(self.value.as_weak_arcish());
        self.update(value)
    }
}

impl<T> std::ops::Deref for VersionedCell<T> {
    type Target = Arcish<T>;

    fn deref(&self) -> &Arcish<T> {
        &self.value
    }
}

#[derive(Debug)]
pub enum Arcish<T> {
    Strong(Arc<T>),
    Weak(Weak<T>),
}

impl<T> Arcish<T> {
    pub fn strong(value: T) -> Arcish<T> {
        Arcish::Strong(Arc::new(value))
    }

    pub fn from_strong(value: Arc<T>) -> Arcish<T> {
        Arcish::Strong(value)
    }

    pub fn weak(value: &Arc<T>) -> Arcish<T> {
        let arc = value.clone();
        let arc = Arc::downgrade(&arc);

        Arcish::Weak(arc)
    }

    pub fn from_weak(weak: Weak<T>) -> Arcish<T> {
        Arcish::Weak(weak)
    }

    pub fn value(&self) -> Arc<T> {
        match self {
            Arcish::Strong(s) => s.clone(),
            Arcish::Weak(s) => s.clone().upgrade().unwrap(),
        }
    }

    pub fn as_weak_arcish(&self) -> Arcish<T> {
        let arc = self.as_weak();
        Arcish::from_weak(arc)
    }

    pub fn as_weak(&self) -> Weak<T> {
        match self {
            Arcish::Strong(s) => {
                let arc = s.clone();
                Arc::downgrade(&arc)
            }

            Arcish::Weak(s) => s.clone(),
        }
    }

    pub fn clone_value(&self) -> T
    where
        T: Clone,
    {
        match self {
            Arcish::Strong(s) => s.as_ref().clone(),
            Arcish::Weak(s) => s.clone().upgrade().unwrap().as_ref().clone(),
        }
    }
}

impl<T> Clone for Arcish<T> {
    fn clone(&self) -> Arcish<T> {
        match self {
            Arcish::Strong(a) => Arcish::Strong(a.clone()),
            Arcish::Weak(a) => Arcish::Weak(a.clone()),
        }
    }
}
