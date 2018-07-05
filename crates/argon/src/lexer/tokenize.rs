use crate::prelude::*;

use super::Tok;
use crate::ir::pos::SpannedItem;
use crate::lexer::Token;
use crate::CompileError;
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

    fn trace(&self, prefix: &str) {
        trace!(target: "argon::tokenize", "input={:?}", self.input);

        trace!(
            target: "argon::tokenize",
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

    fn consume_token(&mut self, size: usize) -> (usize, usize) {
        // get the starting position
        let start_pos = self.start_pos;
        // and advance it to the current position
        self.start_pos = self.pos;

        // reset the token size
        self.token_size = 0;

        let ret = (start_pos, self.pos);

        self.consume(size);

        ret
    }

    fn finalize_current(&mut self, size: usize, next_state: LexerState) -> (usize, &str, usize) {
        let token = &self.token_start[..self.token_size];
        self.token_start = self.rest;
        self.state = next_state;
        let (start_pos, end_pos) = self.consume_token(size);

        self.trace("-");
        trace!(target: "argon::tokenize", "-> token=WS body={:?}", token.clone());
        (start_pos, token, end_pos)
    }
}

impl Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), CompileError>;

    fn next(&mut self) -> Option<Result<(usize, Token, usize), CompileError>> {
        loop {
            let next = {
                let Lexer { state, rest, .. } = self;

                state.next(rest.chars().next(), rest)
            };

            self.trace("+");
            trace!(target: "argon::tokenize", "-> {:?}", next);

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
                    trace!(target: "argon::tokenize", "-> token={:?}", token);
                    return Some(Ok((start, token.spanned(start, end), end)));
                }

                LexerNext::EmitCurrent(size, tok, next_state) => {
                    let (start, token, end) = self.finalize_current(size, next_state);
                    return Some(Ok((start, tok(token).spanned(start, end), end)));
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
enum LexerNext<'a> {
    WholeToken(usize, Tok),
    FinalizeButDontEmitToken(usize, LexerState),
    EmitCurrent(usize, fn(&'a str) -> Tok, LexerState),
    Transition(usize, LexerState),
    Continue(usize),
    EOF,
}

impl LexerNext<'a> {
    fn finalize_no_emit(next_state: LexerState) -> LexerNext<'a> {
        LexerNext::FinalizeButDontEmitToken(1, next_state)
    }

    fn consume() -> LexerNext<'a> {
        LexerNext::Continue(1)
    }

    fn emit(tok: fn(&str) -> Tok, next_state: LexerState) -> LexerNext<'a> {
        LexerNext::EmitCurrent(1, tok, next_state)
    }

    fn transition_to(next_state: LexerState) -> LexerNext<'a> {
        LexerNext::Transition(1, next_state)
    }

    fn reconsume(self) -> LexerNext<'a> {
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

impl<'a> LexerNext<'a> {
    fn emit_token(t: Tok, size: usize) -> LexerNext<'a> {
        LexerNext::WholeToken(size, t)
    }

    fn emit_current(size: usize, tok: fn(&str) -> Tok, next_state: LexerState) -> LexerNext<'a> {
        LexerNext::EmitCurrent(size, tok, next_state)
    }
}

impl LexerState {
    fn next<'input>(&self, c: Option<char>, rest: &'input str) -> Result<LexerNext, CompileError> {
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
                None => LexerNext::emit(tk_id, LexerState::Top).reconsume(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition_to(LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit(tk_id, LexerState::Top).reconsume()
                    }
                }
            },

            LexerState::ContinueIdent => match c {
                None => LexerNext::emit(tk_id, LexerState::Top).reconsume(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::consume()
                    } else {
                        LexerNext::emit(tk_id, LexerState::Top).reconsume()
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

fn tk_int(token: &str) -> Tok {
    Tok::Int(token.parse().unwrap())
}

fn tk_id(token: &str) -> Tok {
    Tok::id(token)
}

fn tk_float(token: &str) -> Tok {
    Tok::Float(F64::from_float(token.parse().unwrap()))
}

struct Matchers {
    keywords: Keywords,
}

impl Matchers {
    fn keywords(keywords: &[(&'static str, Tok)]) -> Matchers {
        Matchers {
            keywords: Keywords::new(keywords.into()),
        }
    }

    fn match_keyword(&self, rest: &str) -> Option<(Tok, usize)> {
        self.keywords.match_keyword(rest)
    }
}

struct Keywords {
    tokens: Vec<(&'static str, Tok, usize)>,
}

impl Keywords {
    fn new(strings: Vec<(&'static str, Tok)>) -> Keywords {
        let tokens = strings.iter().map(|(s, t)| (*s, *t, s.len())).collect();
        Keywords { tokens }
    }

    fn match_keyword(&self, rest: &str) -> Option<(Tok, usize)> {
        for (string, token, len) in &self.tokens {
            if rest.starts_with(string) {
                return Some((*token, *len));
            }
        }

        None
    }
}
