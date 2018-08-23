crate mod block;
crate mod expression;
crate mod function;
crate mod module;
crate mod types;

#[cfg(test)]
mod test_helpers;

crate use self::block::Block;
crate use self::expression::Expression;
crate use self::function::Function;
crate use self::module::Module;
crate use self::types::{InferType, RawTypeVar, TypeEnv, TypeVar};

impl InferType {
    crate fn annotate<T>(self, item: T) -> Annotated<T> {
        Annotated { ty: self, item }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
crate struct Annotated<T> {
    crate ty: InferType,
    crate item: T,
}

impl<T> std::ops::Deref for Annotated<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.item
    }
}
