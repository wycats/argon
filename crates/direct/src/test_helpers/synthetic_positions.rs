#![allow(unused)]

use crate::ast;

pub fn synthesize_source(module: ast::Module<'static>) -> (ast::Module<'static>, String) {
    SourceSynthesizer::new(module).synthesize()
}

#[derive(new)]
pub struct SourceSynthesizer {
    #[new(default)]
    pos: usize,
    #[new(value = "String::new()")]
    source: String,
    module: ast::Module<'static>,
}

impl SourceSynthesizer {
    fn synthesize(self) -> (ast::Module<'static>, String) {
        unimplemented!()
    }
}
