#![allow(unused_imports)]
use crate::prelude::*;

use codespan::CodeMap;

crate trait ErrorFormatter {
    type Display: Display;

    fn format(&self, map: &CodeMap) -> Self::Display;
}
