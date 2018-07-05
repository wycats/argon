use crate::prelude::*;

use crate::database::VersionedCell;

crate enum GetResult<T, E> {
    ValueResult(ValueResult<T>),
    SkipResult(SkipResult<E>),
}

pub enum GetReifyResult<T, E> {
    ValueResult(T),
    SkipResult(SkipResult<E>),
}

impl<T, E> GetReifyResult<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            GetReifyResult::ValueResult(value) => value,
            GetReifyResult::SkipResult(..) => panic!("ZOMG"),
        }
    }
}

impl<T, E> GetResult<T, E> {
    crate fn value(value: T) -> GetResult<T, E> {
        GetResult::ValueResult(ValueResult::NewValue(value))
    }
}

crate enum ValueResult<T> {
    ValidCache,
    NewValue(T),
}

pub enum SkipResult<E> {
    None,
    Error(E),
}

impl<T> std::ops::Try for GetResult<T, Error> {
    type Ok = ValueResult<T>;
    type Error = SkipResult<Error>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        use self::GetResult::*;

        match self {
            ValueResult(value) => Ok(value),
            SkipResult(other) => Err(other),
        }
    }

    fn from_error(err: SkipResult<Error>) -> GetResult<T, Error> {
        GetResult::SkipResult(err)
    }

    fn from_ok(value: ValueResult<T>) -> GetResult<T, Error> {
        GetResult::ValueResult(value)
    }
}

impl<T> std::ops::Try for GetReifyResult<T, Error> {
    type Ok = T;
    type Error = SkipResult<Error>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        use self::GetReifyResult::*;

        match self {
            ValueResult(value) => Ok(value),
            SkipResult(other) => Err(other),
        }
    }

    fn from_error(err: SkipResult<Error>) -> GetReifyResult<T, Error> {
        GetReifyResult::SkipResult(err)
    }

    fn from_ok(value: T) -> GetReifyResult<T, Error> {
        GetReifyResult::ValueResult(value)
    }
}

impl From<NoneError> for SkipResult<Error> {
    fn from(_: NoneError) -> SkipResult<Error> {
        SkipResult::None
    }
}

impl From<std::io::Error> for SkipResult<Error> {
    fn from(err: std::io::Error) -> SkipResult<Error> {
        SkipResult::Error(err.into_error())
    }
}

impl From<Error> for SkipResult<Error> {
    fn from(err: Error) -> SkipResult<Error> {
        SkipResult::Error(err)
    }
}

impl From<crate::parser::ParseError> for SkipResult<Error> {
    fn from(err: crate::parser::ParseError) -> SkipResult<Error> {
        SkipResult::Error(err.into_error())
    }
}

impl<T> From<Error> for GetResult<T, Error> {
    fn from(err: Error) -> GetResult<T, Error> {
        GetResult::SkipResult(SkipResult::Error(err))
    }
}

impl<T> From<Option<T>> for GetResult<T, Error> {
    fn from(value: Option<T>) -> GetResult<T, Error> {
        match value {
            None => GetResult::SkipResult(SkipResult::None),
            Some(value) => GetResult::value(value),
        }
    }
}

crate struct Table<Key: Clone + Ord, Value> {
    entries: Arc<Mutex<BTreeMap<Key, VersionedCell<Value>>>>,
}

impl<Key: Clone + Ord, Value> Table<Key, Value> {
    crate fn new() -> Table<Key, Value> {
        Table {
            entries: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    crate fn get(&self, key: &Key) -> Option<VersionedCell<Value>> {
        let entries = self.entries.lock().unwrap();
        entries.get(key).map(|e| e.weak())
    }

    crate fn get_revision(&self, key: &Key) -> Option<usize> {
        self.entries.lock().unwrap().get(key).map(|e| e.revision())
    }

    crate fn insert(&self, key: Key, value: Value) {
        self.entries
            .lock()
            .unwrap()
            .insert(key, VersionedCell::new(value));
    }

    crate fn insert_shared(&self, key: Key, value: VersionedCell<Value>) -> VersionedCell<Value> {
        let ret = value.weak();
        self.entries.lock().unwrap().insert(key, value);
        ret
    }
}
