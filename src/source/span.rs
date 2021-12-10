//! This module provides ways of tracking ranges (spans) in the source code.

use super::{Location, Source};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Bound, Deref, RangeBounds},
};

/// A span (a range) in the source code.
///
/// See [`Reader::mark`](super::Reader::mark) and
/// [`Reader::span`](super::Reader::span) to create a span.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    /// Start of the span.
    start: Location,
    /// Length of the span in string segments.
    length: usize,
}

impl Span {
    /// Creates a new span given the start location and length.
    pub(super) fn new(start: Location, length: usize) -> Self {
        Self { start, length }
    }

    /// The start location of this span.
    pub fn start(&self) -> Location {
        self.start.clone()
    }

    /// The end location of this span.
    pub fn end(&self) -> Location {
        Location::new(
            self.source().clone(),
            self.start.position() + self.length,
        )
    }

    /// The length of this span in string segments.
    pub fn len(&self) -> usize {
        self.length
    }

    /// The source code object this span refers to.
    pub fn source(&self) -> &Source {
        self.start.source()
    }

    /// Gets the string this span includes as a whole.
    pub fn as_str(&self) -> &str {
        let start = self.start.position();
        self.source().get(start .. start + self.len()).unwrap()
    }

    /// Creates a type that, when displayed, shows the span contents, rather
    /// than location.
    pub fn content(&self) -> SpanContent {
        SpanContent { span: self.clone() }
    }

    /// Slices this span to the given range. Returns `None` if the range is
    /// invalid.
    pub fn try_slice<R>(&self, range: R) -> Option<Self>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(&position) => position,
            Bound::Excluded(position) => position.saturating_add(1),
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(position) => position.saturating_add(1),
            Bound::Excluded(&position) => position,
            Bound::Unbounded => self.len(),
        };

        if start <= self.length && end <= self.length && start <= end {
            let start_loc = Location::new(
                self.start.source().clone(),
                self.start.position() + start,
            );
            Some(Self::new(start_loc, end - start))
        } else {
            None
        }
    }

    /// Slices this span to the given range.
    ///
    /// # Panics
    /// Panics if the range is invalid.
    pub fn slice<R>(&self, range: R) -> Self
    where
        R: RangeBounds<usize> + fmt::Debug + Clone,
    {
        fn bad_slice<R>(range: R) -> !
        where
            R: fmt::Debug,
        {
            panic!("Invalid range {:?} of span", range)
        }

        match self.try_slice(range.clone()) {
            Some(span) => span,
            None => bad_slice(range),
        }
    }

    /// Joins two spans into a larger span, such that both are included, but not
    /// bigger than it needs to be. Note however that if there is a hole between
    /// the two spans, the hole is included.
    ///
    /// # Panic
    /// Panics if the spans have different [`Source`]s.
    pub fn join(&self, other: &Span) -> Span {
        if self.source() != other.source() {
            panic!("Cannot join spans of different sources");
        }
        let self_end = self.start.position() + self.length;
        let other_end = other.start.position() + other.length;
        if self_end <= other_end {
            Span::new(self.start.clone(), other_end - self.start.position())
        } else {
            Span::new(other.start.clone(), self_end - other.start.position())
        }
    }

    /// Expands this span in order to contain the whole lines the original span
    /// contains.
    pub fn expand_lines(&self) -> Span {
        let start_line = self.start().line();
        let end_line = self.end().line();
        let init = self.source().line_start(start_line);
        let end = self
            .source()
            .try_line_start(end_line + 1)
            .unwrap_or(self.source().len());
        Self::new(Location::new(self.source().clone(), init), end - init)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Span")
            .field("source", self.source())
            .field("start", &self.start())
            .field("end", &self.end())
            .field("content", &self.as_str())
            .finish()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let file = self.source().name();
        let (line_start, col_start) = self.start().line_column();
        let (line_end, col_end) = self.end().line_column();
        write!(
            fmtr,
            "in {} from ({}, {}) to ({}, {})",
            file,
            line_start + 1,
            col_start + 1,
            line_end + 1,
            col_end + 1
        )
    }
}

impl<T> AsRef<T> for Span
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.as_str().as_ref()
    }
}

/// A type that, when displayed, shows the span contents, rather than location.
#[derive(Clone, Debug)]
pub struct SpanContent {
    /// The inner span of a source code.
    span: Span,
}

impl SpanContent {
    /// Returns the inner span.
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the span contents as a string.
    pub fn as_str(&self) -> &str {
        self.span.as_str()
    }
}

impl Deref for SpanContent {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SpanContent {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", &**self)
    }
}

impl PartialEq for SpanContent {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T> PartialEq<T> for SpanContent
where
    T: ?Sized,
    str: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        &**self == other
    }
}

impl Eq for SpanContent {}

impl PartialOrd for SpanContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialOrd<T> for SpanContent
where
    T: ?Sized,
    str: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl Ord for SpanContent {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl Hash for SpanContent {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(hasher)
    }
}

impl<T> AsRef<T> for SpanContent
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}

impl<T> Borrow<T> for SpanContent
where
    T: ?Sized,
    str: Borrow<T>,
{
    fn borrow(&self) -> &T {
        (**self).borrow()
    }
}
