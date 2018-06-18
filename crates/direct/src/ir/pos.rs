use std::fmt::{self, Debug};
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(u32);

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[derive(Copy, Clone, Eq, PartialEq, new)]
pub struct ByteSpan {
    start: usize,
    end: usize,
    source: Id,
}

#[derive(Eq, PartialEq, Clone)]
pub struct Spanned<Node: PartialEq + Debug> {
    node: Node,
    span: Span,
}

impl<Node: PartialEq + Debug> fmt::Debug for Spanned<Node> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}@{:?}", self.node, self.span)
        } else {
            write!(f, "{:?}", self.node)
        }
    }
}

impl<Node: PartialEq + Debug> Deref for Spanned<Node> {
    type Target = Node;

    fn deref(&self) -> &Node {
        &self.node
    }
}

crate trait SpannedItem: Sized + PartialEq + Debug {
    fn spanned(self, l: usize, r: usize) -> Spanned<Self>;
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

    fn synthetic(self, desc: &'static str) -> Spanned<T> {
        Spanned {
            node: self,
            span: Span::Synthetic(desc),
        }
    }
}
