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

    fn accumulate(&mut self, size: usize) {
        self.consume(size);
        self.token_size += size;
    }

    fn consume(&mut self, size: usize) {
        self.pos += size;
        self.rest = &self.rest[size..];
    }

    fn consume_token(&mut self) -> (usize, &'input str, usize) {
        // slice off the accumulated token characters
        let token = &self.token_start[..self.token_size];

        // get the starting position
        let start_pos = self.start_pos;
        // and advance it to the current position
        self.start_pos = self.pos;

        // reset the token size
        self.token_size = 0;

        (start_pos, token, self.pos)
    }

    fn finalize_current(
        &mut self,
        size: usize,
        next_state: LexerState,
    ) -> (usize, &'input str, usize) {
        let (start_pos, token, end_pos) = self.consume_token();
        self.consume(size);
        self.token_start = self.rest;
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

                    self.consume(size);
                    self.token_start = self.rest;

                    self.trace("-");
                    trace!(target: "wasm::tokenize", "-> token={:?}", token);
                    return Some(Ok((start, token, end)));
                }

                LexerNext::EmitCurrent(size, tok, next_state) => {
                    let (start, token, end) = self.finalize_current(size, next_state);
                    return Some(Ok((start, tok(token), end)));
                }

                LexerNext::FinalizeButDontEmitToken(size, next_state) => {
                    self.finalize_current(size, next_state);
                    // Parser doesn't handle WS tokens
                    // return Some((0, Tok::WS(token), 0));
                }

                LexerNext::Continue(size) => {
                    self.accumulate(size);

                    self.trace("-");
                }

                LexerNext::Transition(size, state) => {
                    self.accumulate(size);
                    self.state = state;

                    self.trace("-");
                }
            };
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    FinalizeButDontEmitToken(usize, LexerState),
    EmitCurrent(usize, fn(&'input str) -> Tok<'input>, LexerState),
    Transition(usize, LexerState),
    Continue(usize),
    EOF,
}

impl LexerNext<'input> {
    fn finalize_no_emit(next_state: LexerState) -> LexerNext<'input> {
        LexerNext::FinalizeButDontEmitToken(1, next_state)
    }

    fn consume() -> LexerNext<'input> {
        LexerNext::Continue(1)
    }

    fn emit<'i>(tok: fn(&'i str) -> Tok<'i>, next_state: LexerState) -> LexerNext<'i> {
        LexerNext::EmitCurrent(1, tok, next_state)
    }

    fn transition_to(next_state: LexerState) -> LexerNext<'input> {
        LexerNext::Transition(1, next_state)
    }

    fn reconsume(self) -> LexerNext<'input> {
        match self {
            LexerNext::WholeToken(_, tok) => LexerNext::WholeToken(0, tok),
            LexerNext::FinalizeButDontEmitToken(_, tok) => {
                LexerNext::FinalizeButDontEmitToken(0, tok)
            }
            LexerNext::EmitCurrent(_, tok, state) => LexerNext::EmitCurrent(0, tok, state),
            LexerNext::Transition(_, state) => LexerNext::Transition(0, state),
            LexerNext::Continue(_) => LexerNext::Continue(1),
            LexerNext::EOF => LexerNext::EOF,
        }
    }
}

impl<'input> LexerNext<'input> {
    fn emit_token(t: Tok<'input>, size: usize) -> LexerNext<'input> {
        LexerNext::WholeToken(size, t)
    }

    fn emit_current(
        size: usize,
        tok: fn(&'input str) -> Tok<'input>,
        next_state: LexerState,
    ) -> LexerNext<'input> {
        LexerNext::EmitCurrent(size, tok, next_state)
    }
}

impl LexerState {
    fn next<'input>(
        &self,
        c: Option<char>,
        rest: &'input str,
    ) -> Result<LexerNext<'input>, CompileError> {
        let out = match self {
            LexerState::Top => match c {
                None => LexerNext::EOF,
                Some(c) => {
                    if let Some((tok, size)) = MATCHERS.match_keyword(rest) {
                        LexerNext::emit_token(tok, size)
                    } else if c.is_digit(10) {
                        LexerNext::transition_to(LexerState::Integer).reconsume()
                    } else if c.is_whitespace() {
                        LexerNext::transition_to(LexerState::Whitespace)
                    } else if UnicodeXID::is_xid_start(c) {
                        LexerNext::transition_to(LexerState::StartIdent).reconsume()
                    } else {
                        return Err(CompileError::LexError);
                    }
                }
            },

            LexerState::Whitespace => match c {
                None => LexerNext::EOF,
                Some(c) => {
                    if c.is_whitespace() {
                        LexerNext::consume()
                    } else {
                        LexerNext::finalize_no_emit(LexerState::Top).reconsume()
                    }
                }
            },

            LexerState::StartIdent => match c {
                None => LexerNext::emit(Tok::Id, LexerState::Top).reconsume(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition_to(LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit(Tok::Id, LexerState::Top).reconsume()
                    }
                }
            },

            LexerState::ContinueIdent => match c {
                None => LexerNext::emit(Tok::Id, LexerState::Top).reconsume(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::consume()
                    } else {
                        LexerNext::emit(Tok::Id, LexerState::Top).reconsume()
                    }
                }
            },

            LexerState::Integer => match c {
                None => LexerNext::emit_current(0, tk_int, LexerState::Top),
                Some(c) => {
                    if c.is_digit(10) {
                        LexerNext::consume()
                    } else if c == '.' {
                        LexerNext::transition_to(LexerState::Decimal)
                    } else {
                        LexerNext::emit(tk_int, LexerState::Top).reconsume()
                    }
                }
            },

            LexerState::Decimal => match c {
                None => LexerNext::emit_current(0, tk_float, LexerState::Top),
                Some(c) => {
                    if c.is_digit(10) {
                        LexerNext::consume()
                    } else {
                        LexerNext::emit(tk_float, LexerState::Top).reconsume()
                    }
                }
            },
        };

        Ok(out)
    }
}

fn tk_int<'i>(token: &'i str) -> Tok<'i> {
    Tok::Int(token.parse().unwrap())
}

fn tk_float<'i>(token: &'i str) -> Tok<'i> {
    Tok::Float(F64::from_float(token.parse().unwrap()))
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
