use parity_wasm::elements;
use wasmi;

fn invoke(
    module: &elements::Module,
    name: &str,
    args: &[wasmi::RuntimeValue],
) -> Option<wasmi::RuntimeValue> {
    let module = wasmi::Module::from_parity_wasm_module(module.clone()).unwrap();
    let main = wasmi::ModuleInstance::new(&module, &wasmi::ImportsBuilder::default())
        .unwrap()
        .run_start(&mut wasmi::NopExternals)
        .unwrap();

    main.invoke_export(name, args, &mut wasmi::NopExternals)
        .unwrap()
}

macro_rules! syntax {
    ($mod_name:ident { module $syntax:expr; parse as |$module_builder:ident| $parse:expr; invoke $name:ident ($($args:expr),*) = $expected:expr }) => {
        #[allow(unused)]
        mod $mod_name {
            use wasm::test_helpers::*;

            use super::invoke;
            use wasm::{ast, compile_module, ModuleParser};
            use wasm::ir::Type;
            use nan_preserving_float::{F32, F64};

            fn module() -> ast::Module<'static> {
                ModuleParser::new().parse($syntax).unwrap()
            }

            #[test]
            fn test_parse() {
                let expected: Vec<ast::Module<'static>> = 
                    AstBuilder::new().module(|$module_builder| $parse).done();

                assert_eq!(vec![module()], expected);
            }

            #[test]
            fn test_compile() {
                let module = compile_module(&module());
                let value = invoke(&module.unwrap(), stringify!($name), &[$($args),*]);

                assert_eq!(value, $expected);
            }
        }
    }
}

syntax!(noop_function {
    module "export def noop () {}";

    parse as |module| {
        module.function("noop", |f| {
            f.exported()
        })
    };

    invoke noop() = None
});

syntax!(identity_function {
    module "export def id(x: i32) -> i32 { x }";

    parse as |module| {
        module.function("id", |f| {
            f.exported();
            f.param("x", Type::I32);
            f.returning(Type::I32);
            f.expression(|e| e.variable("x"))
        })
    };

    invoke id(wasmi::RuntimeValue::I32(10)) = Some(wasmi::RuntimeValue::I32(10))
});

syntax!(plus_function_i32 {
    module "export def plus(x: i32, y: i32) -> i32 { x + y }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::I32);
            f.param("y", Type::I32);
            f.returning(Type::I32);
            f.expression(|e| e.variable("x") + e.variable("y"));
        })
    };

    invoke plus(wasmi::RuntimeValue::I32(20), wasmi::RuntimeValue::I32(33)) = Some(wasmi::RuntimeValue::I32(53))
});

syntax!(plus_function_f32 {
    module "export def plus(x: f32, y: f32) -> f32 { x + y }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::F32);
            f.param("y", Type::F32);
            f.returning(Type::F32);
            f.expression(|e| e.variable("x") + e.variable("y"));
        })
    };

    invoke plus(
        wasmi::RuntimeValue::F32(F32::from_float(20.0)),
        wasmi::RuntimeValue::F32(F32::from_float(33.0))
    ) = Some(wasmi::RuntimeValue::F32(F32::from_float(53.0)))
});


syntax!(plus_function_i64 {
    module "export def plus(x: i64, y: i64) -> i64 { x + y }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::I64);
            f.param("y", Type::I64);
            f.returning(Type::I64);
            f.expression(|e| e.variable("x") + e.variable("y"));
        })
    };

    invoke plus(wasmi::RuntimeValue::I64(20), wasmi::RuntimeValue::I64(33)) = Some(wasmi::RuntimeValue::I64(53))
});

syntax!(plus_function_f64 {
    module "export def plus(x: f64, y: f64) -> f64 { x + y }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::F64);
            f.param("y", Type::F64);
            f.returning(Type::F64);
            f.expression(|e| e.variable("x") + e.variable("y"));
        })
    };

    invoke plus(
        wasmi::RuntimeValue::F64(F64::from_float(20.0)),
        wasmi::RuntimeValue::F64(F64::from_float(33.0))
    ) = Some(wasmi::RuntimeValue::F64(F64::from_float(53.0)))
});


syntax!(plus_const_left {
    module "export def plus(x: i32) -> i32 { x + 1 }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::I32);
            f.returning(Type::I32);
            f.expression(|e| e.variable("x") + e.i32(1));
        })
    };

    invoke plus(wasmi::RuntimeValue::I32(20)) = Some(wasmi::RuntimeValue::I32(21))
});

syntax!(plus_const_right {
    module "export def plus(x: i32) -> i32 { 1 + x }";

    parse as |module| {
        module.function("plus", |f| {
            f.exported();
            f.param("x", Type::I32);
            f.returning(Type::I32);
            f.expression(|e| e.i32(1) + e.variable("x"));
        })
    };

    invoke plus(wasmi::RuntimeValue::I32(20)) = Some(wasmi::RuntimeValue::I32(21))
});
