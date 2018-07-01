use super::Function;
use crate::infer::unify::UnifyTable;
use crate::infer::Constraints;
use crate::ir::resolved;

#[derive(Debug)]
crate struct Module {
    crate funcs: Vec<Function>,
}

impl Module {
    crate fn from(resolved::Module { funcs }: resolved::Module, vars: &mut UnifyTable) -> Module {
        let funcs = funcs
            .into_iter()
            .map(|func| Function::from(func, vars))
            .collect();

        Module { funcs }
    }

    crate fn constraints(&self) -> Constraints {
        let mut constraints = Constraints::empty();

        for function in &self.funcs {
            constraints += function.constraints();
        }

        constraints
    }
}
