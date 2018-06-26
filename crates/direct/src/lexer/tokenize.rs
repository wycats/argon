use super::Tok;
use unicode_xid::UnicodeXID;

pub struct Lexer<'input> {
    input: &'input str,
    rest: &'input str,
    token_start: &'input str,
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
            pos: 0,
            token_size: 0,
            state: LexerState::Top,
        }
    }

    fn trace(&mut self, prefix: &str) {
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
}

impl Iterator for Lexer<'input> {
    type Item = (usize, Tok<'input>, usize);

    fn next(&mut self) -> Option<(usize, Tok<'input>, usize)> {
        loop {
            let next = {
                let Lexer { state, rest, .. } = self;

                state.next(rest.chars().next(), rest)
            };

            self.trace("+");
            trace!(target: "wasm::tokenize", "-> {:?}", next);

            match next {
                LexerNext::EOF => {
                    self.trace("-");
                    return None;
                }

                LexerNext::WholeToken(size, token) => {
                    self.rest = &self.rest[size..];
                    self.token_start = self.rest;
                    self.pos += size;

                    self.trace("-");
                    trace!(target: "wasm::tokenize", "-> token={:?}", token);
                    return Some((0, token, 0));
                }

                LexerNext::EmitCurrentId(size, next_state) => {
                    let token_size = self.token_size;
                    self.token_size = 0;

                    let token = &self.token_start[..token_size];
                    self.rest = &self.rest[size..];
                    self.token_start = self.rest;
                    self.pos += size;
                    self.state = next_state;

                    self.trace("-");
                    trace!(target: "wasm::tokenize", "-> token={:?}", Tok::Id(token));
                    return Some((0, Tok::Id(token), 0));
                }

                LexerNext::EmitCurrentWs(size, next_state) => {
                    let token_size = self.token_size;
                    self.token_size = 0;

                    let token = &self.token_start[..token_size];
                    self.rest = &self.rest[size..];
                    self.token_start = self.rest;
                    self.pos += size;
                    self.state = next_state;

                    self.trace("-");
                    trace!(target: "wasm::tokenize", "-> token={:?}", Tok::WS(token));
                    // Parser doesn't handle WS tokens
                    // return Some((0, Tok::WS(token), 0));
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
    Whitespace,
}

#[derive(Debug)]
enum LexerNext<'input> {
    WholeToken(usize, Tok<'input>),
    EmitCurrentId(usize, LexerState),
    EmitCurrentWs(usize, LexerState),
    Transition(usize, LexerState),
    EOF,
}

impl LexerNext<'input> {
    fn emit_token(s: &str, t: Tok<'input>) -> LexerNext<'input> {
        LexerNext::WholeToken(s.len(), t)
    }

    fn emit_current_id(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentId(consume, next_state)
    }

    fn emit_current_ws(consume: usize, next_state: LexerState) -> LexerNext<'input> {
        LexerNext::EmitCurrentWs(consume, next_state)
    }

    fn transition(consume: usize, state: LexerState) -> LexerNext<'input> {
        LexerNext::Transition(consume, state)
    }
}

impl LexerState {
    fn next(&self, c: Option<char>, rest: &'input str) -> LexerNext<'input> {
        match self {
            LexerState::Top => match c {
                None => LexerNext::EOF,
                Some(c) => {
                    if rest.starts_with("export") {
                        LexerNext::emit_token("export", Tok::Export)
                    } else if rest.starts_with("def") {
                        LexerNext::emit_token("def", Tok::Def)
                    } else if rest.starts_with("f64") {
                        LexerNext::emit_token("f64", Tok::F64)
                    } else if rest.starts_with("50") {
                        // TODO: HAX / enter number parsing state
                        LexerNext::emit_token("50", Tok::Int(50))
                    } else if rest.starts_with("->") {
                        LexerNext::emit_token("->", Tok::Arrow)
                    } else if c.is_whitespace() {
                        LexerNext::transition(1, LexerState::Whitespace)
                    } else if UnicodeXID::is_xid_start(c) {
                        LexerNext::transition(0, LexerState::StartIdent)
                    } else if c == '(' {
                        LexerNext::emit_token("(", Tok::OpenParen)
                    } else if c == ')' {
                        LexerNext::emit_token(")", Tok::CloseParen)
                    } else if c == '{' {
                        LexerNext::emit_token("{", Tok::OpenBrace)
                    } else if c == '}' {
                        LexerNext::emit_token("}", Tok::CloseBrace)
                    } else if c == ':' {
                        LexerNext::emit_token(":", Tok::Colon)
                    } else if c == '+' {
                        LexerNext::emit_token("+", Tok::Add)
                    } else {
                        unimplemented!("in {:?} with {:?}", self, rest)
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
                None => unimplemented!(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition(1, LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit_current_id(0, LexerState::Top)
                    }
                }
            },

            LexerState::ContinueIdent => match c {
                None => unimplemented!(),
                Some(c) => {
                    if UnicodeXID::is_xid_continue(c) {
                        LexerNext::transition(1, LexerState::ContinueIdent)
                    } else {
                        LexerNext::emit_current_id(0, LexerState::Top)
                    }
                }
            },
        }
    }
}
