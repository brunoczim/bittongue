//! This crate provides utilities to handle source code of a programming
//! language.
//!
//! # Example
//! ```rust
//! use langsrc::Source;
//!
//! # fn main() {
//! let source = Source::new(
//!     "main.c",
//!     "int /* v́ */\nmain(int argc, char const *argv[])\n{\n    return 0;\n}\n"
//! );
//!
//! let mut reader = source.reader();
//! reader.mark();
//! for _ in 0 .. 3 {
//!     assert!(reader.next());
//! }
//!
//! let int_span = reader.span();
//! assert_eq!(&int_span.content(), "int");
//! assert_eq!(int_span.start().pos(), 0);
//! assert_eq!(int_span.start().line_column(), (0, 0));
//! assert_eq!(int_span.end().pos(), 3);
//! assert_eq!(int_span.end().line_column(), (0, 3));
//!
//! for _ in 0 .. 9 {
//!     assert!(reader.next());
//! }
//! reader.mark();
//! for _ in 0 .. 4 {
//!     assert!(reader.next());
//! }
//!
//! let main_span = reader.span();
//! assert_eq!(&main_span.content(), "main");
//! assert_eq!(main_span.start().pos(), 12);
//! assert_eq!(main_span.start().line_column(), (1, 0));
//! assert_eq!(main_span.end().pos(), 16);
//! assert_eq!(main_span.end().line_column(), (1, 4));
//! # }
//! ```

mod indexing;
mod loc;
mod reader;
mod span;

pub use indexing::SourceIndex;
use indexing::{IndexArray, IndexArrayBuilder, IndexArrayIter};
pub use loc::Location;
pub use reader::Reader;
pub use span::{Span, SpanContent};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Index,
    rc::Rc,
};
use unicode_segmentation::UnicodeSegmentation;

/// Inner structure of a source.
#[derive(Debug)]
struct SrcInner {
    /// File name.
    name: Box<str>,
    /// Contents of the source.
    contents: Box<str>,
    /// List of string segmentation in the source.
    segments: IndexArray,
    /// List of newlines in the source.
    newlines: IndexArray,
}

/// A source code object, such as read from a file.
#[derive(Debug, Clone)]
pub struct Source {
    /// The inner structure containing the actual data.
    inner: Rc<SrcInner>,
}

impl Source {
    /// Creates a new source code object given its name and its contentss.
    pub fn new<S0, S1>(name: S0, contents: S1) -> Self
    where
        S0: Into<Box<str>>,
        S1: Into<Box<str>>,
    {
        let name = name.into();
        let contents = contents.into();
        let mut segments = IndexArrayBuilder::new();
        let mut newlines = IndexArrayBuilder::new();

        for (idx, grapheme) in contents.grapheme_indices(true) {
            if grapheme == "\n" {
                newlines.push(segments.len());
            }
            segments.push(idx);
        }
        segments.push(contents.len());

        let segments = segments.into();
        let newlines = newlines.into();
        let inner = SrcInner { name, contents, segments, newlines };
        Self { inner: Rc::new(inner) }
    }

    /// The (file) name of the source.
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// The length the source.
    pub fn len(&self) -> usize {
        self.inner.segments.len() - 1
    }

    /// The contentss of the source.
    pub fn contents(&self) -> &str {
        &self.inner.contents
    }

    /// Iterator over the segments of the source.
    pub fn segments(&self) -> SegmentsIter {
        SegmentsIter { inner: self.inner.segments.iter() }
    }

    /// Iterator over the newlines of the source.
    pub fn newlines(&self) -> NewlinesIter {
        NewlinesIter { inner: self.inner.segments.iter() }
    }

    /// Indexes this source. It can be a single `usize` or a range of `usize`.
    pub fn get<I>(&self, indexer: I) -> Option<&I::Output>
    where
        I: SourceIndex,
    {
        indexer.get(self)
    }

    /// Creates a source code reader (a stream) from this source code object.
    pub fn reader(&self) -> Reader {
        Reader::new(self.clone())
    }
}

impl<I> Index<I> for Source
where
    I: SourceIndex,
{
    type Output = I::Output;

    fn index(&self, indexer: I) -> &Self::Output {
        indexer.index(self)
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Source {}

impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Source {
    fn cmp(&self, other: &Self) -> Ordering {
        (&*self.inner as *const SrcInner).cmp(&(&*other.inner as *const _))
    }
}

impl Hash for Source {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (&*self.inner as *const SrcInner).hash(hasher)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str(self.name())
    }
}

/// Iterator over the segments of a source. Double-ended and sized.
#[derive(Debug)]
pub struct SegmentsIter<'src> {
    /// The inner iterator over the indices.
    inner: IndexArrayIter<'src>,
}

impl<'src> Iterator for SegmentsIter<'src> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<'src> DoubleEndedIterator for SegmentsIter<'src> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'array> ExactSizeIterator for SegmentsIter<'array> {}

/// Iterator over the newlines of a source. Double-ended and sized.
#[derive(Debug)]
pub struct NewlinesIter<'src> {
    /// The inner iterator over the indices.
    inner: IndexArrayIter<'src>,
}

impl<'src> Iterator for NewlinesIter<'src> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<'src> DoubleEndedIterator for NewlinesIter<'src> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'array> ExactSizeIterator for NewlinesIter<'array> {}
