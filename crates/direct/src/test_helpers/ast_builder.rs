use crate::ast;
use crate::ir::shared::{ConstExpression, Type};

#[derive(Debug)]
pub struct AstBuilder {
    modules: Vec<ast::Module<'static>>,
}

impl AstBuilder {
    pub fn new() -> AstBuilder {
        AstBuilder { modules: vec![] }
    }

    pub fn module(self, module: impl FnOnce(&mut ModuleBuilder)) -> AstBuilder {
        let mut builder = ModuleBuilder {
            functions: vec![],
            parent: self,
        };

        module(&mut builder);
        builder.done()
    }

    pub fn done(self) -> Vec<ast::Module<'static>> {
        self.modules
    }
}

#[derive(Debug)]
pub struct ModuleBuilder {
    functions: Vec<ast::Function<'static>>,
    parent: AstBuilder,
}

impl ModuleBuilder {
    pub fn function(&mut self, name: &'static str, function: impl FnOnce(&mut FunctionBuilder)) {
        let mut builder = FunctionBuilder {
            name,
            parameters: vec![],
            expressions: vec![],
            ret: None,
            modifiers: Modifiers::new(),
            parent: self,
        };

        function(&mut builder);
        builder.done()
    }

    fn done(self) -> AstBuilder {
        let ModuleBuilder {
            functions,
            mut parent,
        } = self;

        let module = ast::Module::new(functions);
        parent.modules.push(module);
        parent
    }
}

#[derive(Debug, new)]
pub struct Modifiers {
    #[new(default)]
    export: bool,
}

#[derive(Debug)]
pub struct FunctionBuilder<'module> {
    name: &'static str,
    parameters: Vec<ast::Parameter<'static>>,
    expressions: Vec<ast::Expression<'static>>,
    ret: Option<Type>,
    modifiers: Modifiers,
    parent: &'module mut ModuleBuilder,
}

impl FunctionBuilder<'module> {
    pub fn exported(&mut self) {
        self.modifiers.export = true
    }

    pub fn param(&mut self, name: &'static str, ty: Type) {
        let name = ast::Identifier::new(name);
        self.parameters.push(ast::Parameter::new(name, ty.into()));
    }

    pub fn returning(&mut self, ty: Type) {
        self.ret = Some(ty.into());
    }

    pub fn expression(&mut self, body: impl FnOnce(ExpressionBuilder) -> ast::Expression<'static>) {
        let expr = body(ExpressionBuilder);
        self.expressions.push(expr);
    }

    pub fn done(self) {
        let FunctionBuilder {
            name,
            parameters,
            expressions,
            ret,
            modifiers,
            parent,
        } = self;

        let mut function = ast::Function::new(
            ast::Identifier::new(name),
            ast::Parameters::new(parameters),
            ret.unwrap_or(Type::Void),
            ast::Block::new(expressions),
        );

        if modifiers.export {
            function = function.exported();
        }

        parent.functions.push(function);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ExpressionBuilder;

impl ExpressionBuilder {
    pub fn variable(self, name: &'static str) -> ast::Expression<'static> {
        ast::Expression::VariableAccess(ast::Identifier::new(name))
    }

    pub fn i32(self, integer: i32) -> ast::Expression<'static> {
        ast::Expression::Const(ConstExpression::I32(integer))
    }
}

impl std::ops::Add for ast::Expression<'static> {
    type Output = ast::Expression<'static>;

    fn add(self, rhs: ast::Expression<'static>) -> ast::Expression<'static> {
        ast::Expression::Plus(ast::PlusExpression::new(Box::new(self), Box::new(rhs)))
    }
}
