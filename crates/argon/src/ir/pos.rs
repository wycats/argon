use std::fmt::{self, Debug};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Id(u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Span {
    Synthetic(&'static str),
    ByteSpan(ByteSpan),
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Span::Synthetic(desc) => write!(f, "{}", desc),
            Span::ByteSpan(span) => write!(f, "{}:{}", span.start, span.end),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, new)]
pub struct ByteSpan {
    start: usize,
    end: usize,
    source: Id,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Spanned<Node: PartialEq + Debug> {
    crate node: Node,
    crate span: Span,
}

impl Spanned<&str> {
    crate fn to_spanned_string(&self) -> Spanned<String> {
        Spanned {
            node: self.node.to_string(),
            span: self.span,
        }
    }
}

impl<Node: PartialEq + fmt::Display + fmt::Debug> fmt::Display for Spanned<Node> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}@{:?}", self.node, self.span)
        } else {
            write!(f, "{:?}", self.node)
        }
    }
}

crate trait SpannedItem: Sized + PartialEq + Debug {
    fn spanned(self, l: usize, r: usize) -> Spanned<Self>;
    fn copy_span<T: PartialEq + Debug>(self, item: &Spanned<T>) -> Spanned<Self>;
    fn synthetic(self, desc: &'static str) -> Spanned<Self>;
}

impl<T: PartialEq + Debug> SpannedItem for T {
    fn spanned(self, l: usize, r: usize) -> Spanned<T> {
        Spanned {
            node: self,
            span: Span::ByteSpan(ByteSpan {
                start: l,
                end: r,
                source: Id(0),
            }),
        }
    }

    fn copy_span<U: PartialEq + Debug>(self, item: &Spanned<U>) -> Spanned<Self> {
        Spanned {
            node: self,
            span: item.span,
        }
    }

    fn synthetic(self, desc: &'static str) -> Spanned<T> {
        Spanned {
            node: self,
            span: Span::Synthetic(desc),
        }
    }
}
