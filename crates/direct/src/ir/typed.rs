use crate::ir::{ast, ConstExpression, FunctionModifiers, Spanned, Type, TypeError};
use crate::{resolved, MathOperator, MathType};
use nan_preserving_float::{F32, F64};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Const(ConstExpression),

    VariableAccess(u32),
    Binary {
        operator: MathOperator,
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
}

impl Expression {
    crate fn typed(self, ty: Type) -> TypedExpression {
        TypedExpression::new(self, ty)
    }
}

#[derive(Debug, PartialEq)]
pub struct TypedModule<'input> {
    pub funcs: Vec<TypedFunction<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct TypedFunction<'input> {
    pub name: Spanned<&'input str>,
    pub params: Vec<Type>,
    pub symbols: Vec<Spanned<&'input str>>,
    pub ret: Type,
    pub body: TypedBlock,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug, PartialEq)]
pub struct TypedBlock {
    pub expressions: Vec<TypedExpression>,
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

impl resolved::Module<'input> {
    crate fn ast_to_hir(mut self) -> Result<TypedModule<'input>, TypeError> {
        let mut funcs = vec![];

        for func in self.funcs.drain(..) {
            funcs.push(func.ast_to_hir()?);
        }

        Ok(TypedModule { funcs })
    }
}

impl resolved::Function<'input> {
    fn ast_to_hir(self) -> Result<TypedFunction<'input>, TypeError> {
        let name = self.name;

        let ret = self.ret;
        let params = self.params;
        let symbols = self.symbols;

        let mut expressions = vec![];

        for expr in &self.body.expressions {
            expressions.push(expr.ast_to_hir(None, &params)?);
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
impl resolved::Expression {
    fn ast_to_hir(
        &self,
        hint: Option<Type>,
        params: &[Type],
    ) -> Result<TypedExpression, TypeError> {
        let expr = match self {
            resolved::Expression::Const(constant) => match constant {
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
                        Expression::Const(ConstExpression::F32(F32::from_float(*int as f32))),
                        Type::Math(MathType::F32),
                    ),

                    Some(Type::Math(MathType::F64)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F64(F64::from_float(*int as f64))),
                        Type::Math(MathType::F64),
                    ),

                    rest => unimplemented!(),
                },

                ast::ConstExpression::Float(float) => match hint {
                    Some(Type::Math(MathType::F32)) => TypedExpression::new(
                        Expression::Const(ConstExpression::F32(F32::from_float(
                            float.to_float() as f32
                        ))),
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

            resolved::Expression::VariableAccess(id) => {
                let ty = params[*id as usize].clone();
                let expr = Expression::VariableAccess(*id);

                TypedExpression::new(expr, ty)
            }

            resolved::Expression::Binary { operator, lhs, rhs } => {
                typed_binary(*operator, lhs, rhs, params)?
            }
        };

        Ok(expr)
    }
}

fn typed_binary(
    operator: MathOperator,
    lhs: &resolved::Expression,
    rhs: &resolved::Expression,
    params: &[Type],
) -> Result<TypedExpression, TypeError> {
    let lhs = lhs.ast_to_hir(None, params)?;
    let rhs = rhs.ast_to_hir(Some(lhs.ty.clone()), params)?;

    let lty = lhs.ty.clone();
    let rty = rhs.ty.clone();

    if lty == rty {
        let ty = lty.clone();
        let expr = Expression::Binary {
            operator,
            lhs: box lhs,
            rhs: box rhs,
        };

        Ok(TypedExpression::new(expr, ty))
    } else {
        return Err(TypeError::MismatchedBinary(operator, lty, rty));
    }
}
