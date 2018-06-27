pub mod annotated;
pub mod ast;
pub mod pos;
pub mod resolved;
crate mod shared;

crate use self::annotated::{InferType, TypeVar};
pub use self::pos::*;
pub use self::shared::{
    CompileError, ConstExpression, FunctionModifiers, FunctionType, Type, TypeError,
};
