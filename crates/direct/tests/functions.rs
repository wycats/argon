use parity_wasm::elements;
use wasm::ir::Type;
use wasm::test_helpers::AstBuilder;
use wasm::ModuleParser;
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

syntax!(plus_function {
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

// mod plus_function {
//     use super::invoke;
//     use wasm::ir::Type;
//     use wasm::test_helpers::*;
//     use wasm::{ast, compile_module, ModuleParser};

//     fn module() -> ast::Module<'static> {
//         ModuleParser::new()
//             .parse("export def plus(x: i32, y: i32) -> i32 { x + y }")
//             .unwrap()
//     }

//     fn expected() -> Vec<ast::Module<'static>> {
//         AstBuilder::new()
//             .module(|m| {
//                 m.function("plus", |f| {
//                     f.exported();
//                     f.param("x", Type::I32);
//                     f.param("y", Type::I32);
//                     f.returning(Type::I32);
//                     f.expression(|e| e.variable("x") + e.variable("y"));
//                 });
//             })
//             .done()
//     }

//     #[test]
//     fn test_parse() {
//         assert_eq!(vec![module()], expected());
//     }

//     #[test]
//     fn test_compile() {
//         let module = compile_module(&module());
//         let value = invoke(
//             &module,
//             "plus",
//             &[wasmi::RuntimeValue::I32(20), wasmi::RuntimeValue::I32(33)],
//         );

//         assert_eq!(value, Some(wasmi::RuntimeValue::I32(53)));
//     }

// }

#[test]
fn parse_addition_function() {
    let parsed = ModuleParser::new()
        .parse("def foo(x: i32, y: i32) -> i32 { x + y }")
        .unwrap();

    let expected = AstBuilder::new()
        .module(|m| {
            m.function("foo", |f| {
                f.param("x", Type::I32);
                f.param("y", Type::I32);
                f.returning(Type::I32);
                f.expression(|e| e.variable("x") + e.variable("y"));
            });
        })
        .done();

    assert_eq!(vec![parsed], expected);
}

#[test]
fn parse_left_increment_function() {
    let parsed = ModuleParser::new()
        .parse("def foo(x: i32) -> i32 { x + 1 }")
        .unwrap();

    let expected = AstBuilder::new()
        .module(|m| {
            m.function("foo", |f| {
                f.param("x", Type::I32);
                f.returning(Type::I32);
                f.expression(|e| e.variable("x") + e.i32(1));
            });
        })
        .done();

    assert_eq!(vec![parsed], expected);
}

#[test]
fn parse_right_increment_function() {
    let parsed = ModuleParser::new()
        .parse("def foo(x: i32) -> i32 { 1 + x }")
        .unwrap();

    let expected = AstBuilder::new()
        .module(|m| {
            m.function("foo", |f| {
                f.param("x", Type::I32);
                f.returning(Type::I32);
                f.expression(|e| e.i32(1) + e.variable("x"));
            });
        })
        .done();

    assert_eq!(vec![parsed], expected);
}
