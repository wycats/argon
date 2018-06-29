use argon::Span;

pub struct ReadablePositionSpan {
    original: Span,
    line: usize,
    column: usize,
    source: String,
}

pub enum ReadableSpan {
    Position(ReadablePositionSpan),
    Synthetic(&'static str),
}

fn convert_span(span: &Span) -> ReadableSpan {
    match span {
        Span::Synthetic(desc) => ReadableSpan::Synthetic(desc),

        _ => unimplemented!(),
    }
}
