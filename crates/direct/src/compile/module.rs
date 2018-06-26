use super::function::compile_function;
use crate::infer::unify::UnifyTable;
use crate::ir::{annotated, CompileError};
use crate::{ast, resolved};
use parity_wasm::{builder, elements};

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

pub fn compile_module(input: &ast::Module) -> Result<elements::Module, CompileError> {
    let mut table = UnifyTable::new();

    let module = resolved::resolve_module_names(input)?;
    let module = annotated::Module::from(module, &mut table);
    trace!(target: "wasm::compile::module", "Module: {:#?}", module);
    let constraints = module.constraints();
    trace!(target: "wasm::compile::constraints", "Constraints: {:#?}", constraints);
    let substitutions = table.unify(constraints)?;

    trace!(target: "wasm::compile::substitutions", "Substitutions: {:#?}", substitutions);
    let module = substitutions.apply_module(module);
    trace!(target: "wasm::compile::applies", "After Substitutions: {:#?}", module);

    let mut builder = builder::module();

    for func in &module.funcs {
        let function = builder::function();
        let function = compile_function(function, func);
        let location: CodeLocation =
            unsafe { std::mem::transmute(builder.push_function(function)) };

        if func.modifiers.export {
            builder = builder
                .export()
                .field(func.name.node)
                .internal()
                .func(location.signature)
                .build();
        }
    }

    Ok(builder.build())
}
