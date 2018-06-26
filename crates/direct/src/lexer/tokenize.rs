use super::Tok;
use crate::CompileError;
use nan_preserving_float::F64;
use unicode_xid::UnicodeXID;

lazy_static! {
    static ref MATCHERS: Matchers = {
        Matchers::keywords(&[
            ("export", Tok::Export),
            ("def", Tok::Def),
            ("i32", Tok::I32),
            ("i64", Tok::I64),
            ("u32", Tok::U32),
            ("u64", Tok::U64),
            ("f32", Tok::F32),
            ("f64", Tok::F64),
            ("->", Tok::Arrow),
            ("(", Tok::OpenParen),
            (")", Tok::CloseParen),
            ("{", Tok::OpenBrace),
            ("}", Tok::CloseBrace),
            (":", Tok::Colon),
            ("+", Tok::Add),
            ("-", Tok::Sub),
            ("*", Tok::Mul),
            ("/", Tok::Div),
            (",", Tok::Comma),
        ])
    };
}

pub struct Lexer<'input> {
    input: &'input str,
    rest: &'input str,
    token_start: &'input str,
    start_pos: usize,
    token_size: usize,
    pos: usize,
    state: LexerState,
}

impl Lexer<'input> {
    crate fn new(input: &'input str) -> Lexer<'input> {
        Lexer {
            input,
            rest: input,
            token_start: input,
            start_pos: 0,
            pos: 0,
            token_size: 0,
            state: LexerState::Top,
        }
    }

    fn trace(&mut self, prefix: &str) {
        trace!(target: "wasm::tokenize", "input={:?}", self.input);

        trace!(
            target: "wasm::tokenize",
            "{} rest={:?} token-start={:?} token-size={:?} state={:?}",
            prefix,
            self.rest,
            self.token_start,
            self.token_size,
            self.state
        );
    }

    fn finalize_current(
        &mut self,
        size: usize,
        next_state: LexerState,
    ) -> (usize, &'input str, usize) {
        let token_size = self.token_size;
        let start_pos = self.start_pos;
        let end_pos = self.pos;

        self.token_size = 0;
        self.start_pos = self.pos;

        let token = &self.token_start[..token_size];
        self.rest = &self.rest[size..];
        self.token_start = self.rest;
        self.pos += size;
        self.state = next_state;

        self.trace("-");
        trace!(target: "wasm::tokenize", "-> token={:?}", Tok::WS(token));
        (start_pos, token, end_pos)
    }
}

impl Iterator for Lexer<'input> {
    type Item = Result<(usize, Tok<'input>, usize), CompileError>;

    fn next(&mut self) -> Option<Result<(usize, Tok<'input>, usize), CompileError>> {
        loop {
            let next = {
                let Lexer { state, rest, .. } = self;

                state.next(rest.chars().next(), rest)
            };

            self.trace("+");
            trace!(target: "wasm::tokenize", "-> {:?}", next);

            let next = match next {
                Ok(n) => n,
                Err(e) => return Some(Err(e)),
            };

            match next {
                LexerNext::EOF => {
                    self.trace("-");
                    return None;
                }

                LexerNext::WholeToken(size, token) => {
                    let start = self.start_pos;
                    let end = self.pos + size;

                    self.rest = &self.rest[size..];
                    self.token_start = self.rest;
                    self.pos += size;

                    self.trace("-");
                    trace!(target: "wasm::tokenize", "-> token={:?}", token);
                    return Some(Ok((start, token, end)));
                }

                LexerNext::EmitCurrentId(size, next_state) => {
                    let (start, token, end) = self.finalize_current(size, next_state);
                    return Some(Ok((start, Tok::Id(token), end)));
                }

                LexerNext::EmitCurrentWs(size, next_state) => {
                    self.finalize_current(size, next_state);
                    // Parser doesn't handle WS tokens
                    // return Some((0, Tok::WS(token), 0));
                }

                LexerNext::EmitCurrentInt(size, next_state) => {
                    let (start, token, end) = self.finalize_current(size, next_state);
                    return Some(Ok((start, Tok::Int(token.parse().unwrap()), end)));
                }

                LexerNext::EmitCurrentDecimal(size, next_state) => {
                    let (start, token, end) = self.finalize_current(size, next_state);
                    return Some(Ok((
                        start,
                        Tok::Float(F64::from_float(token.parse().unwrap())),
                        end,
                    )));
                }

                LexerNext::Transition(consume, state) => {
                    self.pos += consume;
                    self.token_size += consume;
                    self.rest = &self.rest[consume..];
                    self.state = state;

                    self.trace("-");
                }
            };
        }
    }
}

#[derive(Debug)]
enum LexerState {
    Top,
    StartIdent,
    ContinueIdent,
    Integer,
    Decimal,
    Whitespace,
}

#[derive(Debug)]
enum LexerNext<'input> {
    WholeToken(usize, Tok<'input>),
    EmitCurrentId(usize, LexerState),
    EmitCurrentWs(usize, LexerState),
    EmitCurrentInt(usize, LexerState),
    EmitCurrentDecimal(usize, LexerState),
    Transition(usize, LexerState),
    EOF,
}

impl LexerNext<'input> {
    fn emit_token(t: Tok<'input>, size: usize) -> LexerNext<'input> {
        LexerNext::WholeToken(size, t)
    }

    fn emit_current_id(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentId(consume, next_state)
    }

    fn emit_current_ws(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentWs(consume, next_state)
    }

    fn emit_current_int(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentInt(consume, next_state)
    }

    fn emit_current_decimal(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentDecimal(consume, next_state)
    }

    fn transition(consume: usize, state: LexerState) -> LexerNext<'input> {
        LexerNext::Transition(consume, state)
    }
}

impl LexerState {
    fn next(&self, c: Option<char>, rest: &'input str) -> Result<LexerNext<'input>, CompileError> {
        let out = match self {
            LexerState::Top => match c {
                None => LexerNext::EOF,
                Some(c) => {
                    if let Some((tok, size)) = MATCHERS.match_keyword(rest) {
                        LexerNext::emit_token(tok, size)
                    } else if c.is_digit(10) {
                        LexerNext::transition(0, LexerState::Integer)
                    } else if c.is_whitespace() {
                        LexerNext::transition(1, LexerState::Whitespace)
                    } else if UnicodeXID::is_xid_start(c) {
                        LexerNext::transition(0, LexerState::StartIdent)
                    } else {
                        return Err(CompileError::LexError);
                    }
                }
            },

            LexerState::Whitespace => match c {
                None => LexerNext::EOF,
                Some(c) => {
                    if c.is_whitespace() {
                        LexerNext::transition(1, LexerState::Whitespace)
                    } else {
                        LexerNext::emit_current_ws(0, LexerState::Top)
                    }
                }
            },

            LexerState::StartIdent => match c {
                None => LexerNext::emit_current_id(0, LexerState::Top),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition(1, LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit_current_id(0, LexerState::Top)
                    }
                }
            },

            LexerState::ContinueIdent => match c {
                None => LexerNext::emit_current_id(0, LexerState::Top),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition(1, LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit_current_id(0, LexerState::Top)
                    }
                }
            },

            LexerState::Integer => match c {
                None => LexerNext::emit_current_int(0, LexerState::Top),
                Some(c) => {
                    if c.is_digit(10) {
                        LexerNext::transition(1, LexerState::Integer)
                    } else if c == '.' {
                        LexerNext::transition(1, LexerState::Decimal)
                    } else {
                        LexerNext::emit_current_int(0, LexerState::Top)
                    }
                }
            },

            LexerState::Decimal => match c {
                None => LexerNext::emit_current_decimal(0, LexerState::Top),
                Some(c) => {
                    if c.is_digit(10) {
                        LexerNext::transition(1, LexerState::Decimal)
                    } else {
                        LexerNext::emit_current_decimal(0, LexerState::Top)
                    }
                }
            },
        };

        Ok(out)
    }
}

struct Matchers {
    keywords: Keywords,
}

impl Matchers {
    fn keywords(keywords: &[(&'static str, Tok<'static>)]) -> Matchers {
        Matchers {
            keywords: Keywords::new(keywords.into()),
        }
    }

    fn match_keyword(&self, rest: &str) -> Option<(Tok<'static>, usize)> {
        self.keywords.match_keyword(rest)
    }
}

struct Keywords {
    tokens: Vec<(&'static str, Tok<'static>, usize)>,
}

impl Keywords {
    fn new(strings: Vec<(&'static str, Tok<'static>)>) -> Keywords {
        let tokens = strings.iter().map(|(s, t)| (*s, *t, s.len())).collect();
        Keywords { tokens }
    }

    fn match_keyword(&self, rest: &str) -> Option<(Tok<'static>, usize)> {
        for (string, token, len) in &self.tokens {
            if rest.starts_with(string) {
                return Some((*token, *len));
            }
        }

        None
    }
}
