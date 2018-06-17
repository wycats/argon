extern crate parity_wasm;
extern crate wabt;
extern crate wasmi;

fn main() {
    let wasm_binary: Vec<u8> = wabt::wat2wasm(
        r#"
            (module
            (func $id (param $lhs i32) (result i32)
                get_local $lhs)
            (export "id" (func $id))
            )
	   "#,
    ).expect("failed to parse wat");

    let module: parity_wasm::elements::Module =
        parity_wasm::deserialize_buffer(&wasm_binary).expect("failed to load wasm");

    println!("{:#?}", module);
}

/*
Module {
    magic: 1836278016,
    version: 1,
    sections: [
        Type(
            TypeSection(
                [
                    Function(
                        FunctionType {
                            form: 96,
                            params: [
                                I32
                            ],
                            return_type: Some(
                                I32
                            )
                        }
                    )
                ]
            )
        ),
        Function(
            FunctionSection(
                [
                    Func(
                        0
                    )
                ]
            )
        ),
        Export(
            ExportSection(
                [
                    ExportEntry {
                        field_str: "id",
                        internal: Function(
                            0
                        )
                    }
                ]
            )
        ),
        Code(
            CodeSection(
                [
                    FuncBody {
                        locals: [],
                        opcodes: Opcodes(
                            [
                                GetLocal(
                                    0
                                ),
                                End
                            ]
                        )
                    }
                ]
            )
        )
    ]
}
*/
