use crate::prelude::*;
use crate::CompileError;

use crate::database::VersionedCell;

#[must_use]
pub enum GetResult<T, E = ArgonError> {
    Value(T),
    SkipResult(SkipResult<E>),
}

impl<T, E> GetResult<T, E> {
    crate fn value(value: T) -> GetResult<T, E> {
        GetResult::Value(value)
    }
}

pub enum SkipResult<E> {
    None,
    Error(E),
}

macro_rules! max_revision {
    ($($input:ident)*) => {{
        let mut max: usize = 0;

        $(
            max = std::cmp::max(max, $input.revision());
        )*

        max
    }}
}

macro_rules! validate_cache {
    (input: $input:expr, $cache:ident) => {{
        validate_cache_keyword!($cache);
        validate_cache!(input: $input, cache: $cache)
    }};

    (inputs: [$($input:expr),*], $cache:ident) => {{
        validate_cache_keyword!($cache);
        validate_cache!(inputs: [$($input),*], cache: $cache)
    }};

    (input: $input:expr,cache: $cache:expr) => {{
        let input = $input;
        match $cache {
            None => input,
            Some(cache) => {
                if input.revision() == cache.revision() {
                    return GetResult::Value(cache);
                } else {
                    input
                }
            }
        }
    }};

    (inputs: [$($input:expr),*],cache: $cache:expr) => {{
        let mut max: usize = 0;

        let tuple = ($({
            let input = $input;
            max = std::cmp::max(max, input.revision());
            input
        },)*);

        match $cache {
            None => tuple,
            Some(cache) => {
                if max == cache.revision() {
                    return GetResult::Value(cache);
                } else {
                    tuple
                }
            }
        }
    }};
}

macro_rules! validate {
    (index: $index:expr, key: $key:expr, input: $input:expr, cache: $cache:expr, |$id:ident| { $($body:tt)* }) => {{
        let input = validate_cache!(input: $input, cache: $cache);

        let cell = input.derive();

        let value = {
            let $id = input;

            $($body)*
        };

        let value = cell.owned(value);
        let value = $index.insert_shared($key.clone(), value);

        $crate::database::GetResult::value(value)
    }};

    (index: $index:expr, key: $key:expr, inputs: [$($input:ident),*], cache: $cache:expr, { $($body:tt)* }) => {{
        let revision = max_revision!($($input)*);
        let ($($input),*) = validate_cache!(inputs: [$($input),*], cache: $cache);

        let cell = $crate::database::derive_from_revision(revision);

        let value = {
            $(
                let $input = $input;
            )*

            $($body)*
        };

        let value = cell.owned(value);
        let value = $index.insert_shared($key.clone(), value);

        $crate::database::GetResult::value(value)
    }};

    (
        $index:tt[$key:ident] = $func:ident($input:ident)
    ) => {{
        let key = $key;
        let cache = $index.get(key);
        validate!(index: $index, key: $key, input: $input, cache: cache, |id| { $func(&id)? })
    }};

    (
        $index:tt[$key:ident] = $func:ident($($input:ident),*)
    ) => {{
        let key = $key;
        let cache = $index.get(key);
        validate!(index: $index, key: $key, inputs: [$($input),*], cache: cache, { $func($(&$input),*)? })
    }};
}

impl<T> std::ops::Try for GetResult<T, ArgonError> {
    type Ok = T;
    type Error = SkipResult<ArgonError>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        use self::GetResult::*;

        match self {
            Value(value) => Ok(value),
            SkipResult(other) => Err(other),
        }
    }

    fn from_error(err: SkipResult<ArgonError>) -> GetResult<T, ArgonError> {
        GetResult::SkipResult(err)
    }

    fn from_ok(value: T) -> GetResult<T, ArgonError> {
        GetResult::value(value)
    }
}
impl From<CompileError> for SkipResult<ArgonError> {
    fn from(error: CompileError) -> SkipResult<ArgonError> {
        SkipResult::Error(ArgonError::from(error))
    }
}

impl From<NoneError> for SkipResult<ArgonError> {
    fn from(_: NoneError) -> SkipResult<ArgonError> {
        SkipResult::None
    }
}

impl From<std::io::Error> for SkipResult<ArgonError> {
    fn from(err: std::io::Error) -> SkipResult<ArgonError> {
        SkipResult::Error(err.into_error())
    }
}

impl From<codespan::SpanError> for SkipResult<ArgonError> {
    fn from(err: codespan::SpanError) -> SkipResult<ArgonError> {
        SkipResult::Error(ArgonError::from(err))
    }
}

impl<T> From<Option<T>> for GetResult<T, ArgonError> {
    fn from(value: Option<T>) -> GetResult<T, ArgonError> {
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

    crate fn insert_shared(&self, key: Key, value: VersionedCell<Value>) -> VersionedCell<Value> {
        let ret = value.weak();
        self.entries.lock().unwrap().insert(key, value);
        ret
    }
}
