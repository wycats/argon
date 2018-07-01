use super::{Annotated, Block, TypeEnv};
use crate::infer::unify::UnifyTable;
use crate::infer::Constraints;
use crate::ir::{resolved, Spanned};
use crate::{FunctionModifiers, Type};

#[derive(Debug)]
crate struct Function {
    crate name: Spanned<String>,
    crate params: Vec<Spanned<Type>>,
    crate symbols: Vec<Spanned<String>>,
    crate ret: Spanned<Type>,
    crate body: Annotated<Block>,
    crate modifiers: FunctionModifiers,
}

impl Function {
    crate fn from(
        resolved::Function {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }: resolved::Function,
        vars: &mut UnifyTable,
    ) -> Function {
        let body = {
            let env = TypeEnv::params(&params);
            Block::from(body, vars, &env)
        };

        Function {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }
    }

    crate fn constraints(&self) -> Constraints {
        self.body.constraints()
    }
}
