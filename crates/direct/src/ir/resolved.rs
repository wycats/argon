use super::annotated;
use crate::{ast, FunctionModifiers, MathOperator, Spanned, Type};

#[derive(Debug)]
pub struct Module<'input> {
    pub funcs: Vec<Function<'input>>,
}

#[derive(Debug)]
pub struct Function<'input> {
    pub name: Spanned<&'input str>,
    pub params: Vec<Type>,
    pub symbols: Vec<Spanned<&'input str>>,
    pub ret: Type,
    pub body: Block,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(u32),
    Binary {
        operator: MathOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Expression {
    crate fn annotate(
        self,
        vars: &mut annotated::TypeVars,
        env: &annotated::TypeEnv,
    ) -> annotated::Annotated<annotated::Expression> {
        match self {
            Expression::Const(expr) => vars.annotate_fresh(annotated::Expression::Const(expr)),
            Expression::VariableAccess(id) => {
                let ty = env.get_local(id as usize);
                annotated::InferType::Resolved(ty)
                    .annotate(annotated::Expression::VariableAccess(id))
            }
            Expression::Binary {
                operator,
                box lhs,
                box rhs,
            } => {
                let t1 = vars.fresh();
                t1.annotate(annotated::Expression::Binary {
                    operator,
                    lhs: box lhs.annotate(vars, env),
                    rhs: box rhs.annotate(vars, env),
                })
            }
        }
    }
}

crate fn resolve_module_names(
    module: &'input ast::Module<'input>,
) -> Result<Module<'input>, ResolveError> {
    let resolver = ResolveModule { module };
    resolver.resolve()
}

struct ResolveModule<'input> {
    module: &'input ast::Module<'input>,
}

impl ResolveModule<'input> {
    fn resolve(&self) -> Result<Module<'input>, ResolveError> {
        let funcs: Result<Vec<Function<'input>>, ResolveError> = self
            .module
            .funcs
            .iter()
            .map(|func| self.resolve_function(func))
            .collect();

        Ok(Module { funcs: funcs? })
    }

    fn resolve_function(
        &self,
        func: &'input ast::Function<'input>,
    ) -> Result<Function<'input>, ResolveError> {
        ResolveFunction { func }.resolve()
    }
}

struct ResolveFunction<'module> {
    func: &'module ast::Function<'module>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ResolveError {}

impl ResolveFunction<'module> {
    fn resolve(&self) -> Result<Function<'module>, ResolveError> {
        let ResolveFunction { func, .. } = self;

        let mut symbols = vec![];
        let mut params = vec![];

        for (name, ty) in func.args.iter() {
            symbols.push(name);
            params.push(ty.clone());
        }

        let ret = func.ret.clone();

        let mut expressions = vec![];

        for expr in &func.body.expressions {
            expressions.push(self.resolve_expression(expr)?);
        }

        Ok(Function {
            name: func.name.as_ref(),
            params,
            symbols,
            ret,
            body: Block { expressions },
            modifiers: func.modifiers,
        })
    }

    fn resolve_expression(&self, expr: &ast::Expression) -> Result<Expression, ResolveError> {
        let expr = match expr {
            ast::Expression::Const(constant) => Expression::Const(*constant),
            ast::Expression::VariableAccess(id) => {
                let local = self.func.mappings.get(id.node.name).unwrap();
                Expression::VariableAccess(*local)
            }
            ast::Expression::Binary(operator, box ast::BinaryExpression { lhs, rhs }) => {
                let lhs = self.resolve_expression(lhs)?;
                let rhs = self.resolve_expression(rhs)?;

                Expression::Binary {
                    operator: *operator,
                    lhs: box lhs,
                    rhs: box rhs,
                }
            }
        };

        Ok(expr)
    }
}
