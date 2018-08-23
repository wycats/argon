use crate::prelude::*;

use super::annotated;
use codespan::ByteSpan;
use crate::lexer::Token;
use crate::CompileError;
use crate::{ast, FunctionModifiers, MathOperator, Span, Spanned, SpannedItem, Type, UnifyTable};
use failure::Fail;

#[derive(Debug)]
pub struct Module {
    pub funcs: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Spanned<Type>>,
    pub symbols: Vec<Token>,
    pub ret: Spanned<Type>,
    pub body: Spanned<Block>,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(Spanned<usize>),
    Binary {
        operator: Spanned<MathOperator>,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Span for Expression {
    fn span(&self) -> ByteSpan {
        match self {
            Expression::Const(constant) => constant.span(),
            Expression::VariableAccess(id) => id.span(),
            Expression::Binary {
                box lhs, box rhs, ..
            } => lhs.span().to(rhs.span()),
        }
    }
}

impl Expression {
    crate fn annotate(
        self,
        vars: &mut UnifyTable,
        env: &annotated::TypeEnv<'_>,
    ) -> annotated::Annotated<annotated::Expression> {
        match self {
            Expression::Const(expr) => {
                vars.annotate_fresh(annotated::Expression::Const(expr), expr.span())
            }
            Expression::VariableAccess(id) => {
                let ty = env.get_local(id.node);
                annotated::InferType::Resolved(ty)
                    .annotate(annotated::Expression::VariableAccess(id))
            }
            Expression::Binary {
                operator,
                box lhs,
                box rhs,
            } => {
                let t1 = vars.fresh(lhs.span().to(rhs.span()));
                t1.annotate(annotated::Expression::Binary {
                    operator,
                    lhs: box lhs.annotate(vars, env),
                    rhs: box rhs.annotate(vars, env),
                })
            }
        }
    }
}

crate fn resolve_module_names(module: &ast::Module) -> Result<Module, CompileError> {
    let resolver = ResolveModule { module };
    resolver.resolve()
}

struct ResolveModule<'a> {
    module: &'a ast::Module,
}

impl ResolveModule<'a> {
    fn resolve(&self) -> Result<Module, CompileError> {
        let funcs: Result<Vec<Function>, CompileError> = self
            .module
            .funcs
            .iter()
            .map(|func| self.resolve_function(func))
            .collect();

        Ok(Module { funcs: funcs? })
    }

    fn resolve_function(&self, func: &'input ast::Function) -> Result<Function, CompileError> {
        ResolveFunction { func }.resolve()
    }
}

struct ResolveFunction<'a> {
    func: &'a ast::Function,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResolveError {}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResolveError")
    }
}

impl Fail for ResolveError {}

impl<'a> ResolveFunction<'a> {
    fn resolve(&self) -> Result<Function, CompileError> {
        let ResolveFunction { func, .. } = self;

        let mut symbols = vec![];
        let mut params = vec![];

        for (name, ty) in func.args.iter() {
            symbols.push(name);
            params.push(ty.clone());
        }

        let ret = func.ret.clone();

        let mut expressions = vec![];

        for expr in &func.body.node.expressions {
            expressions.push(self.resolve_expression(expr)?);
        }

        Ok(Function {
            name: func.name,
            params,
            symbols,
            ret,
            body: Block { expressions }.copy_span(&func.body),
            modifiers: func.modifiers,
        })
    }

    fn resolve_expression(&self, expr: &ast::Expression) -> Result<Expression, ResolveError> {
        let expr = match expr {
            ast::Expression::Const(constant) => Expression::Const(*constant),
            ast::Expression::VariableAccess(id) => {
                let local = *self.func.mappings.get(&id.to_ident()).unwrap();
                Expression::VariableAccess(local.copy_span(id))
            }
            ast::Expression::Binary(operator, tok, box ast::BinaryExpression { lhs, rhs }) => {
                let lhs = self.resolve_expression(lhs)?;
                let rhs = self.resolve_expression(rhs)?;

                Expression::Binary {
                    operator: (*operator).copy_span(&tok),
                    lhs: box lhs,
                    rhs: box rhs,
                }
            }
        };

        Ok(expr)
    }
}
