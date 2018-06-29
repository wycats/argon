// auto-generated: "lalrpop 0.15.2"
// sha256: 52353a7ceef3abc4fc4ac9c8caf4f9223b87b2bb04cdc5d172d2e4e6ba
use crate::ast;
use crate::ast::*;
use crate::ir::*;
use crate::MathOperator;
use crate::lexer::Tok;
use nan_preserving_float::F64;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(rust_2018_idioms)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
mod __parse__Module {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast;
    use crate::ast::*;
    use crate::ir::*;
    use crate::MathOperator;
    use crate::lexer::Tok;
    use nan_preserving_float::F64;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Tok<'input>),
        Variant1(F64),
        Variant2(&'input str),
        Variant3(i32),
        Variant4(Parameter<'input>),
        Variant5(::std::vec::Vec<Parameter<'input>>),
        Variant6(Expression<'input>),
        Variant7(::std::vec::Vec<Expression<'input>>),
        Variant8(::std::vec::Vec<Function<'input>>),
        Variant9(usize),
        Variant10(Block<'input>),
        Variant11(Function<'input>),
        Variant12(Spanned<RawIdentifier<'input>>),
        Variant13(Module<'input>),
        Variant14(Parameters<'input>),
        Variant15(RawIdentifier<'input>),
        Variant16(Type),
        Variant17(Spanned<Tok<'input>>),
        Variant18(Spanned<F64>),
        Variant19(Spanned<i32>),
        Variant20(Spanned<Type>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -61, 0, -61, -61, 0, -61, 0, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61,
        // State 10
        -35, 0, -35, -35, 0, -35, 0, -35, -35, -35, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35,
        // State 11
        -48, 0, -48, -48, 0, -48, 0, -48, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0,
        // State 14
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 50, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0,
        // State 21
        0, -62, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0,
        // State 22
        0, -63, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0,
        // State 24
        0, -53, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0,
        // State 25
        0, -54, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0,
        // State 26
        0, -49, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0,
        // State 27
        0, -50, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0,
        // State 28
        0, -51, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0,
        // State 29
        0, -52, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 31
        0, 0, -26, -26, 0, -26, 0, -26, 0, -26, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 32
        0, 0, -27, -27, 0, -27, 0, -27, 0, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 33
        0, 0, 0, -29, 0, -29, 0, 0, 0, -29, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29,
        // State 34
        0, 0, 0, -30, 0, -30, 0, 0, 0, -30, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9,
        // State 36
        0, 0, -25, -25, 0, -25, 0, -25, 0, -25, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25,
        // State 37
        0, 0, 57, -28, 0, -28, 0, 58, 0, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28,
        // State 38
        0, 0, 0, 61, 0, 62, 0, 0, 0, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21,
        // State 39
        0, 0, -24, -24, 0, -24, 0, -24, 0, -24, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24,
        // State 40
        0, 0, -23, -23, 0, -23, 0, -23, 0, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23,
        // State 41
        0, 0, -22, -22, 0, -22, 0, -22, 0, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22,
        // State 42
        0, 0, -31, -31, 0, -31, 0, -31, 0, -31, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31,
        // State 43
        0, 0, -34, -34, 0, -34, 0, -34, 0, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34,
        // State 44
        0, 0, -59, -59, 0, -59, 0, -59, 0, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59,
        // State 45
        0, 0, -60, -60, 0, -60, 0, -60, 0, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 0, 0,
        // State 48
        0, 64, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 12, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -44, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, -15, -15, 0, -15, 0, -15, 0, -15, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15,
        // State 67
        0, 0, -16, -16, 0, -16, 0, -16, 0, -16, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16,
        // State 68
        0, 0, 57, -17, 0, -17, 0, 58, 0, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 69
        0, 0, 57, -18, 0, -18, 0, 58, 0, -18, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18,
        // State 70
        0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -42,
        // State 1
        -37,
        // State 2
        -40,
        // State 3
        -43,
        // State 4
        -64,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -41,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -36,
        // State 13
        0,
        // State 14
        0,
        // State 15
        -33,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -19,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -32,
        // State 52
        0,
        // State 53
        -20,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 20, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 23, 24, 0,
        // State 17
        0, 0, 0, 0, 0, 31, 0, 0, 0, 32, 33, 34, 35, 0, 36, 37, 38, 39, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 34, 35, 0, 53, 37, 38, 39, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 60, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 23, 63, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 66, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 0, 0, 0, 0, 37, 69, 0, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 0, 0, 0, 0, 37, 70, 0, 40, 0, 41, 42, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 43, 44, 11, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 71, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###""/""###,
            r###"":""###,
            r###""Float""###,
            r###""Id""###,
            r###""Int""###,
            r###""WS""###,
            r###""def""###,
            r###""export""###,
            r###""f32""###,
            r###""f64""###,
            r###""i32""###,
            r###""i64""###,
            r###""u32""###,
            r###""u64""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 23)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ModuleParser {
        _priv: (),
    }

    impl ModuleParser {
        pub fn new() -> ModuleParser {
            ModuleParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=crate::CompileError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Module<'input>, __lalrpop_util::ParseError<usize, Tok<'input>, crate::CompileError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Tok::OpenParen if true => 0,
                    Tok::CloseParen if true => 1,
                    Tok::Mul if true => 2,
                    Tok::Add if true => 3,
                    Tok::Comma if true => 4,
                    Tok::Sub if true => 5,
                    Tok::Arrow if true => 6,
                    Tok::Div if true => 7,
                    Tok::Colon if true => 8,
                    Tok::Float(_) if true => 9,
                    Tok::Id(_) if true => 10,
                    Tok::Int(_) if true => 11,
                    Tok::WS(_) if true => 12,
                    Tok::Def if true => 13,
                    Tok::Export if true => 14,
                    Tok::F32 if true => 15,
                    Tok::F64 if true => 16,
                    Tok::I32 if true => 17,
                    Tok::I64 if true => 18,
                    Tok::U32 if true => 19,
                    Tok::U64 if true => 20,
                    Tok::OpenBrace if true => 21,
                    Tok::CloseBrace if true => 22,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 23 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Tok::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Tok::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Tok::Mul => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Tok::Add => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Tok::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Tok::Sub => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Tok::Arrow => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Tok::Div => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Tok::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Tok::Float(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Tok::Id(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Tok::Int(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Tok::WS(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Tok::Def => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Tok::Export => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Tok::F32 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Tok::F64 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ Tok::I32 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ Tok::I64 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ Tok::U32 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ Tok::U64 => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ Tok::OpenBrace => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ Tok::CloseBrace => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Module<'input>,__lalrpop_util::ParseError<usize, Tok<'input>, crate::CompileError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                // __Module = Module => ActionFn(0);
                let __sym0 = __pop_Variant13(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Block<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, F64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Function<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Module<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Parameter<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Parameters<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RawIdentifier<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<F64>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<RawIdentifier<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Tok<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<Type>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant20(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Spanned<i32>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant19(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expression<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Function<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Parameter<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ("," <Parameter>) = ",", Parameter => ActionFn(48);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ("," <Parameter>)* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ("," <Parameter>)* = ("," <Parameter>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ("," <Parameter>)+ = ",", Parameter => ActionFn(60);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ("," <Parameter>)+ = ("," <Parameter>)+, ",", Parameter => ActionFn(61);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action61::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<Expression>) = Expression => ActionFn(44);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<Expression>)* =  => ActionFn(42);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action42::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<Expression>)* = (<Expression>)+ => ActionFn(43);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<Expression>)+ = Expression => ActionFn(64);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<Expression>)+ = (<Expression>)+, Expression => ActionFn(65);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<ModifiedFunction*>) =  => ActionFn(68);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action68::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // (<ModifiedFunction*>) = ModifiedFunction+ => ActionFn(69);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // @L =  => ActionFn(59);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action59::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // @R =  => ActionFn(58);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action58::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Binary<Expression2, Spanned<"*">, Expression1> = Expression2, Spanned<"*">, Expression1 => ActionFn(84);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Binary<Expression2, Spanned<"/">, Expression1> = Expression2, Spanned<"/">, Expression1 => ActionFn(85);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Binary<Expression3, Spanned<"+">, Expression2> = Expression3, Spanned<"+">, Expression2 => ActionFn(86);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Binary<Expression3, Spanned<"-">, Expression2> = Expression3, Spanned<"-">, Expression2 => ActionFn(87);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(66);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 13)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Block = "{", (<Expression>)+, "}" => ActionFn(67);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 13)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression = Expression3 => ActionFn(17);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression1 = Identifier => ActionFn(18);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression1 = I32 => ActionFn(19);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression1 = F64 => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression2 = Expression1 => ActionFn(21);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression2 = Binary<Expression2, Spanned<"*">, Expression1> => ActionFn(22);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression2 = Binary<Expression2, Spanned<"/">, Expression1> => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression3 = Expression2 => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression3 = Binary<Expression3, Spanned<"+">, Expression2> => ActionFn(25);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expression3 = Binary<Expression3, Spanned<"-">, Expression2> => ActionFn(26);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // F64 = Spanned<"Float"> => ActionFn(30);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Function = "def", Identifier, Parameters, "->", Type, Block => ActionFn(4);
        let __sym5 = __pop_Variant10(__symbols);
        let __sym4 = __pop_Variant20(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action4::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (6, __symbol, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Function = "def", Identifier, Parameters, Block => ActionFn(5);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant14(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (4, __symbol, 19)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // I32 = Spanned<"Int"> => ActionFn(29);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier = Spanned<RawIdentifier> => ActionFn(27);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction = "export", Function => ActionFn(2);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 22)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction = Function => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction* =  => ActionFn(50);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action50::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 23)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction* = ModifiedFunction+ => ActionFn(51);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction+ = ModifiedFunction => ActionFn(52);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ModifiedFunction+ = ModifiedFunction+, ModifiedFunction => ActionFn(53);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (2, __symbol, 24)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Module =  => ActionFn(70);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action70::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 25)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Module = ModifiedFunction+ => ActionFn(71);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Parameter = Identifier, ":", Type => ActionFn(8);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 26)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Parameters = "(", Parameter, ")" => ActionFn(62);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action62::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (3, __symbol, 27)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Parameters = "(", Parameter, ("," <Parameter>)+, ")" => ActionFn(63);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (4, __symbol, 27)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Parameters = "(", ")" => ActionFn(7);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 27)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawIdentifier = "Id" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "i32" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "i64" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "u32" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "u64" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "f32" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // RawType = "f64" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"*"> = "*" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"+"> = "+" => ActionFn(89);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"-"> = "-" => ActionFn(90);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 32)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"/"> = "/" => ActionFn(91);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 33)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"Float"> = "Float" => ActionFn(92);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant18(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<"Int"> = "Int" => ActionFn(93);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action93::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant19(__nt), __end);
        (1, __symbol, 35)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<RawIdentifier> = RawIdentifier => ActionFn(94);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action94::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 36)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Spanned<RawType> = RawType => ActionFn(95);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant20(__nt), __end);
        (1, __symbol, 37)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Type = Spanned<RawType> => ActionFn(9);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant20(__nt), __end);
        (1, __symbol, 38)
    }
}
pub use self::__parse__Module::ModuleParser;

fn __action0<
    'input,
>(
    (_, __0, _): (usize, Module<'input>, usize),
) -> Module<'input>
{
    (__0)
}

fn __action1<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Function<'input>>, usize),
) -> Module<'input>
{
    Module::new(__0)
}

fn __action2<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Function<'input>, usize),
) -> Function<'input>
{
    __0.exported()
}

fn __action3<
    'input,
>(
    (_, __0, _): (usize, Function<'input>, usize),
) -> Function<'input>
{
    (__0)
}

fn __action4<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, Spanned<RawIdentifier<'input>>, usize),
    (_, args, _): (usize, Parameters<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, ty, _): (usize, Spanned<Type>, usize),
    (_, body, _): (usize, Block<'input>, usize),
) -> Function<'input>
{
    Function::new(name, args, ty, body)
}

fn __action5<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, Spanned<RawIdentifier<'input>>, usize),
    (_, args, _): (usize, Parameters<'input>, usize),
    (_, body, _): (usize, Block<'input>, usize),
) -> Function<'input>
{
    Function::new(name, args, Type::void(), body)
}

fn __action6<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Parameter<'input>, usize),
    (_, __1, _): (usize, ::std::vec::Vec<Parameter<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Parameters<'input>
{
    Parameters::from_parser(__0, __1)
}

fn __action7<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
    (_, __1, _): (usize, Tok<'input>, usize),
) -> Parameters<'input>
{
    Parameters::empty()
}

fn __action8<
    'input,
>(
    (_, __0, _): (usize, Spanned<RawIdentifier<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __1, _): (usize, Spanned<Type>, usize),
) -> Parameter<'input>
{
    Parameter::new(__0, __1)
}

fn __action9<
    'input,
>(
    (_, __0, _): (usize, Spanned<Type>, usize),
) -> Spanned<Type>
{
    __0
}

fn __action10<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::i32()
}

fn __action11<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::i64()
}

fn __action12<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::u32()
}

fn __action13<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::u64()
}

fn __action14<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::f32()
}

fn __action15<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::f64()
}

fn __action16<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, ::std::vec::Vec<Expression<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Block<'input>
{
    Block::new(__0)
}

fn __action17<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action18<
    'input,
>(
    (_, __0, _): (usize, Spanned<RawIdentifier<'input>>, usize),
) -> Expression<'input>
{
    Expression::VariableAccess(__0)
}

fn __action19<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action20<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action21<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action22<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action23<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action24<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action25<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action26<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action27<
    'input,
>(
    (_, id, _): (usize, Spanned<RawIdentifier<'input>>, usize),
) -> Spanned<RawIdentifier<'input>>
{
    id
}

fn __action28<
    'input,
>(
    (_, id, _): (usize, &'input str, usize),
) -> RawIdentifier<'input>
{
    ident(id)
}

fn __action29<
    'input,
>(
    (_, __0, _): (usize, Spanned<i32>, usize),
) -> Expression<'input>
{
    Expression::Const(ast::ConstExpression::Integer(__0))
}

fn __action30<
    'input,
>(
    (_, __0, _): (usize, Spanned<F64>, usize),
) -> Expression<'input>
{
    Expression::Const(ast::ConstExpression::Float(__0))
}

fn __action31<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, F64, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<F64>
{
    rule.spanned(l, r)
}

fn __action32<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, i32, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<i32>
{
    rule.spanned(l, r)
}

fn __action33<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, RawIdentifier<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<RawIdentifier<'input>>
{
    rule.spanned(l, r)
}

fn __action34<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expression<'input>, usize),
    (_, op, _): (usize, Spanned<Tok<'input>>, usize),
    (_, rhs, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expression<'input>
{
    Expression::binary(op, Box::new(BinaryExpression::new(lhs, rhs)))
}

fn __action35<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    rule.spanned(l, r)
}

fn __action36<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expression<'input>, usize),
    (_, op, _): (usize, Spanned<Tok<'input>>, usize),
    (_, rhs, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expression<'input>
{
    Expression::binary(op, Box::new(BinaryExpression::new(lhs, rhs)))
}

fn __action37<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    rule.spanned(l, r)
}

fn __action38<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expression<'input>, usize),
    (_, op, _): (usize, Spanned<Tok<'input>>, usize),
    (_, rhs, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expression<'input>
{
    Expression::binary(op, Box::new(BinaryExpression::new(lhs, rhs)))
}

fn __action39<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    rule.spanned(l, r)
}

fn __action40<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, lhs, _): (usize, Expression<'input>, usize),
    (_, op, _): (usize, Spanned<Tok<'input>>, usize),
    (_, rhs, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Expression<'input>
{
    Expression::binary(op, Box::new(BinaryExpression::new(lhs, rhs)))
}

fn __action41<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    rule.spanned(l, r)
}

fn __action42<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expression<'input>>
{
    vec![]
}

fn __action43<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Expression<'input>>, usize),
) -> ::std::vec::Vec<Expression<'input>>
{
    v
}

fn __action44<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    (__0)
}

fn __action45<
    'input,
>(
    (_, l, _): (usize, usize, usize),
    (_, rule, _): (usize, Type, usize),
    (_, r, _): (usize, usize, usize),
) -> Spanned<Type>
{
    rule.spanned(l, r)
}

fn __action46<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Parameter<'input>>
{
    vec![]
}

fn __action47<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Parameter<'input>>, usize),
) -> ::std::vec::Vec<Parameter<'input>>
{
    v
}

fn __action48<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Parameter<'input>, usize),
) -> Parameter<'input>
{
    (__0)
}

fn __action49<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Function<'input>>, usize),
) -> ::std::vec::Vec<Function<'input>>
{
    (__0)
}

fn __action50<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Function<'input>>
{
    vec![]
}

fn __action51<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Function<'input>>, usize),
) -> ::std::vec::Vec<Function<'input>>
{
    v
}

fn __action52<
    'input,
>(
    (_, __0, _): (usize, Function<'input>, usize),
) -> ::std::vec::Vec<Function<'input>>
{
    vec![__0]
}

fn __action53<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Function<'input>>, usize),
    (_, e, _): (usize, Function<'input>, usize),
) -> ::std::vec::Vec<Function<'input>>
{
    { let mut v = v; v.push(e); v }
}

fn __action54<
    'input,
>(
    (_, __0, _): (usize, Parameter<'input>, usize),
) -> ::std::vec::Vec<Parameter<'input>>
{
    vec![__0]
}

fn __action55<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Parameter<'input>>, usize),
    (_, e, _): (usize, Parameter<'input>, usize),
) -> ::std::vec::Vec<Parameter<'input>>
{
    { let mut v = v; v.push(e); v }
}

fn __action56<
    'input,
>(
    (_, __0, _): (usize, Expression<'input>, usize),
) -> ::std::vec::Vec<Expression<'input>>
{
    vec![__0]
}

fn __action57<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Expression<'input>>, usize),
    (_, e, _): (usize, Expression<'input>, usize),
) -> ::std::vec::Vec<Expression<'input>>
{
    { let mut v = v; v.push(e); v }
}

fn __action58<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

fn __action59<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

fn __action60<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Parameter<'input>, usize),
) -> ::std::vec::Vec<Parameter<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        __temp0,
    )
}

fn __action61<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Parameter<'input>>, usize),
    __1: (usize, Tok<'input>, usize),
    __2: (usize, Parameter<'input>, usize),
) -> ::std::vec::Vec<Parameter<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        __0,
        __temp0,
    )
}

fn __action62<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Parameter<'input>, usize),
    __2: (usize, Tok<'input>, usize),
) -> Parameters<'input>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __temp0,
        __2,
    )
}

fn __action63<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Parameter<'input>, usize),
    __2: (usize, ::std::vec::Vec<Parameter<'input>>, usize),
    __3: (usize, Tok<'input>, usize),
) -> Parameters<'input>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action47(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __temp0,
        __3,
    )
}

fn __action64<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
) -> ::std::vec::Vec<Expression<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __temp0,
    )
}

fn __action65<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Expression<'input>>, usize),
    __1: (usize, Expression<'input>, usize),
) -> ::std::vec::Vec<Expression<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action44(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __temp0,
    )
}

fn __action66<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Tok<'input>, usize),
) -> Block<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action42(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        __0,
        __temp0,
        __1,
    )
}

fn __action67<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, ::std::vec::Vec<Expression<'input>>, usize),
    __2: (usize, Tok<'input>, usize),
) -> Block<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        __0,
        __temp0,
        __2,
    )
}

fn __action68<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Function<'input>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        __temp0,
    )
}

fn __action69<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Function<'input>>, usize),
) -> ::std::vec::Vec<Function<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        __temp0,
    )
}

fn __action70<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Module<'input>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action68(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action71<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Function<'input>>, usize),
) -> Module<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action69(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action72<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
    __3: (usize, usize, usize),
) -> Expression<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action73<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
    __3: (usize, usize, usize),
) -> Expression<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action74<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
    __3: (usize, usize, usize),
) -> Expression<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action75<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
    __3: (usize, usize, usize),
) -> Expression<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action76<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __temp0,
        __0,
        __1,
    )
}

fn __action77<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
        __0,
        __1,
    )
}

fn __action78<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __0,
        __1,
    )
}

fn __action79<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        __temp0,
        __0,
        __1,
    )
}

fn __action80<
    'input,
>(
    __0: (usize, F64, usize),
    __1: (usize, usize, usize),
) -> Spanned<F64>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __temp0,
        __0,
        __1,
    )
}

fn __action81<
    'input,
>(
    __0: (usize, i32, usize),
    __1: (usize, usize, usize),
) -> Spanned<i32>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __temp0,
        __0,
        __1,
    )
}

fn __action82<
    'input,
>(
    __0: (usize, RawIdentifier<'input>, usize),
    __1: (usize, usize, usize),
) -> Spanned<RawIdentifier<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __temp0,
        __0,
        __1,
    )
}

fn __action83<
    'input,
>(
    __0: (usize, Type, usize),
    __1: (usize, usize, usize),
) -> Spanned<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action59(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
        __0,
        __1,
    )
}

fn __action84<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action85<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action86<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action87<
    'input,
>(
    __0: (usize, Expression<'input>, usize),
    __1: (usize, Spanned<Tok<'input>>, usize),
    __2: (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action88<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        __0,
        __temp0,
    )
}

fn __action89<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        __0,
        __temp0,
    )
}

fn __action90<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        __0,
        __temp0,
    )
}

fn __action91<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
) -> Spanned<Tok<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        __0,
        __temp0,
    )
}

fn __action92<
    'input,
>(
    __0: (usize, F64, usize),
) -> Spanned<F64>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        __0,
        __temp0,
    )
}

fn __action93<
    'input,
>(
    __0: (usize, i32, usize),
) -> Spanned<i32>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        __0,
        __temp0,
    )
}

fn __action94<
    'input,
>(
    __0: (usize, RawIdentifier<'input>, usize),
) -> Spanned<RawIdentifier<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        __0,
        __temp0,
    )
}

fn __action95<
    'input,
>(
    __0: (usize, Type, usize),
) -> Spanned<Type>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Tok<'input>, usize) {
    type Error = crate::CompileError;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),crate::CompileError> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Tok<'input>, usize),crate::CompileError> {
    type Error = crate::CompileError;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),crate::CompileError> {
        value
    }
}
