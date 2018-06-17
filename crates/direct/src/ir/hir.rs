use crate::ir::{ast, ConstExpression, FunctionModifiers, Type, TypeError};

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Const(ConstExpression),

    VariableAccess(u32),
    Plus(Box<BinaryExpression>),
    Minus(Box<BinaryExpression>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypedModule<'input> {
    pub funcs: Vec<TypedFunction<'input>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypedFunction<'input> {
    pub name: &'input str,
    pub params: Vec<Type>,
    pub symbols: Vec<&'input str>,
    pub ret: Type,
    pub body: TypedBlock,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypedBlock {
    expressions: Vec<TypedExpression>,
}

impl TypedBlock {
    crate fn iter(&self) -> impl Iterator<Item = &TypedExpression> {
        self.expressions.iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpression {
    pub lhs: TypedExpression,
    pub rhs: TypedExpression,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, new)]
pub struct TypedExpression {
    pub expression: Expression,
    pub ty: Type,
}

impl ast::Module<'input> {
    crate fn ast_to_hir(&self) -> Result<TypedModule, TypeError> {
        let mut funcs = vec![];

        for func in &self.funcs {
            funcs.push(func.ast_to_hir(self)?);
        }

        Ok(TypedModule { funcs })
    }
}

impl ast::Function<'input> {
    fn ast_to_hir(&self, enclosing_module: &ast::Module) -> Result<TypedFunction, TypeError> {
        let name = &self.name.name;

        let mut symbols = vec![];
        let mut params = vec![];

        for (name, ty) in self.args.iter() {
            symbols.push(name);
            params.push(*ty);
        }

        let ret = self.ret;

        let mut expressions = vec![];

        for expr in &self.body.expressions {
            expressions.push(expr.ast_to_hir(self, enclosing_module)?);
        }

        Ok(TypedFunction {
            name,
            params,
            symbols,
            ret,
            body: TypedBlock { expressions },
            modifiers: self.modifiers,
        })
    }
}

#[allow(unused)]
impl ast::Expression<'input> {
    fn ast_to_hir(
        &self,
        enclosing_function: &ast::Function,
        enclosing_module: &ast::Module,
    ) -> Result<TypedExpression, TypeError> {
        let expr = match self {
            ast::Expression::Const(constant) => match constant {
                c @ ConstExpression::I32(_) => {
                    TypedExpression::new(Expression::Const(*c), Type::I32)
                }

                c @ ConstExpression::I64(_) => {
                    TypedExpression::new(Expression::Const(*c), Type::I64)
                }

                c @ ConstExpression::F32(_) => {
                    TypedExpression::new(Expression::Const(*c), Type::F32)
                }

                c @ ConstExpression::F64(_) => {
                    TypedExpression::new(Expression::Const(*c), Type::F64)
                }
            },

            ast::Expression::VariableAccess(id) => {
                let local = enclosing_function.mappings.get(id.name).unwrap();
                let expr = Expression::VariableAccess(*local);

                let arg = &enclosing_function.args.args[*local as usize];

                TypedExpression::new(expr, arg.ty)
            }

            ast::Expression::Plus(ast::BinaryExpression { lhs, rhs }) => {
                let lhs = lhs.ast_to_hir(enclosing_function, enclosing_module)?;
                let rhs = rhs.ast_to_hir(enclosing_function, enclosing_module)?;

                if lhs.ty == rhs.ty {
                    let ty = lhs.ty;
                    let expr = Expression::Plus(Box::new(BinaryExpression { lhs, rhs }));

                    TypedExpression::new(expr, ty)
                } else {
                    return Err(TypeError::MismatchedPlus(lhs.ty, rhs.ty));
                }
            }

            ast::Expression::Minus(ast::BinaryExpression { lhs, rhs }) => {
                let lhs = lhs.ast_to_hir(enclosing_function, enclosing_module)?;
                let rhs = rhs.ast_to_hir(enclosing_function, enclosing_module)?;

                if lhs.ty == rhs.ty {
                    let ty = lhs.ty;
                    let expr = Expression::Minus(Box::new(BinaryExpression { lhs, rhs }));

                    TypedExpression::new(expr, ty)
                } else {
                    return Err(TypeError::MismatchedMinus(lhs.ty, rhs.ty));
                }
            }
        };

        Ok(expr)
    }
}
