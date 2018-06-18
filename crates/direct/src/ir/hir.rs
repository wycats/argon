use crate::ir::{ast, ConstExpression, FunctionModifiers, Type, TypeError};
use crate::{MathOperator, MathType};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Const(ConstExpression),

    VariableAccess(u32),
    Binary(MathOperator, Box<BinaryExpression>),
}

#[derive(Debug, PartialEq)]
pub struct TypedModule<'input> {
    pub funcs: Vec<TypedFunction<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct TypedFunction<'input> {
    pub name: &'input str,
    pub params: Vec<Type>,
    pub symbols: Vec<&'input str>,
    pub ret: Type,
    pub body: TypedBlock,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug, PartialEq)]
pub struct TypedBlock {
    expressions: Vec<TypedExpression>,
}

impl TypedBlock {
    crate fn iter(&self) -> impl Iterator<Item = &TypedExpression> {
        self.expressions.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub lhs: TypedExpression,
    pub rhs: TypedExpression,
}

#[allow(unused)]
#[derive(Debug, PartialEq, new)]
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
            expressions.push(expr.ast_to_hir(None, self, enclosing_module)?);
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
        hint: Option<Type>,
        enclosing_function: &ast::Function,
        enclosing_module: &ast::Module,
    ) -> Result<TypedExpression, TypeError> {
        let expr = match self {
            ast::Expression::Const(constant) => match constant {
                ast::ConstExpression::Integer(int) => match hint {
                    Some(Type::Math(MathType::I32)) => TypedExpression::new(
                        Expression::Const(ConstExpression::I32(*int as i32)),
                        Type::Math(MathType::I32),
                    ),

                    Some(Type::Math(MathType::I64)) => TypedExpression::new(
                        Expression::Const(ConstExpression::I64(*int)),
                        Type::Math(MathType::I64),
                    ),

                    Some(Type::Math(MathType::U32)) => TypedExpression::new(
                        Expression::Const(ConstExpression::U32(*int as u32)),
                        Type::Math(MathType::U32),
                    ),

                    Some(Type::Math(MathType::U64)) => TypedExpression::new(
                        Expression::Const(ConstExpression::U64(*int as u64)),
                        Type::Math(MathType::U64),
                    ),

                    Some(Type::Math(MathType::F32)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F32(*int as f32)),
                        Type::Math(MathType::F32),
                    ),

                    Some(Type::Math(MathType::F64)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F64(*int as f64)),
                        Type::Math(MathType::F64),
                    ),

                    rest => unimplemented!(),
                },

                ast::ConstExpression::Float(float) => match hint {
                    Some(Type::Math(MathType::F32)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F32(*float as f32)),
                        Type::Math(MathType::F32),
                    ),

                    Some(Type::Math(MathType::F64)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F64(*float)),
                        Type::Math(MathType::F64),
                    ),

                    rest => unimplemented!(),
                },

                rest => unimplemented!(),
            },

            ast::Expression::VariableAccess(id) => {
                let local = enclosing_function.mappings.get(id.name).unwrap();
                let expr = Expression::VariableAccess(*local);

                let arg = &enclosing_function.args.args[*local as usize];

                TypedExpression::new(expr, arg.ty)
            }

            ast::Expression::Binary(operator, box ast::BinaryExpression { lhs, rhs }) => {
                typed_binary(*operator, lhs, rhs, enclosing_function, enclosing_module)?
            }
        };

        Ok(expr)
    }
}

fn typed_binary(
    operator: MathOperator,
    lhs: &ast::Expression,
    rhs: &ast::Expression,
    enclosing_function: &ast::Function,
    enclosing_module: &ast::Module,
) -> Result<TypedExpression, TypeError> {
    let lhs = lhs.ast_to_hir(None, enclosing_function, enclosing_module)?;
    let rhs = rhs.ast_to_hir(Some(lhs.ty), enclosing_function, enclosing_module)?;

    if lhs.ty == rhs.ty {
        let ty = lhs.ty;
        let expr = Expression::Binary(operator, Box::new(BinaryExpression { lhs, rhs }));

        Ok(TypedExpression::new(expr, ty))
    } else {
        return Err(TypeError::MismatchedBinary(operator, lhs.ty, rhs.ty));
    }
}
