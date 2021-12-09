//! This module provides means of tracking location in a source code.

use super::{Source, Span};
use std::fmt;

/// The location in a source code.
///
/// See [`Reader::mark`](super::Reader::mark) and
/// [`Reader::location`](super::Reader::location) to create a location.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    /// The source code object.
    source: Source,
    /// The string segment position.
    position: usize,
}

impl Location {
    /// Creates a new location given a source code object and a string segment
    /// position in the object.
    pub(super) fn new(source: Source, position: usize) -> Self {
        Self { source, position }
    }

    /// The string segment position in the source code.
    pub fn position(&self) -> usize {
        self.position
    }

    /// The source code object this location refers to.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Finds the line and column (respectively) of this location in the source
    /// code.
    pub fn line_column(&self) -> (usize, usize) {
        match self.source.inner.newlines.binary_search(self.position) {
            Ok(0) | Err(0) => (0, self.position),
            Ok(n) | Err(n) => {
                (n, self.position - self.source.inner.newlines.index(n - 1) - 1)
            },
        }
    }

    /// Finds the line of this location in the source code.
    pub fn line(&self) -> usize {
        match self.source.inner.newlines.binary_search(self.position) {
            Ok(n) | Err(n) => n,
        }
    }

    /// Finds the column of this location in the source code.
    pub fn column(&self) -> usize {
        let (_, column) = self.line_column();
        column
    }

    /// Creates a [`Span`](Span) containing the whole line this
    /// location is in.
    pub fn line_span(&self) -> Span {
        let line = self.line();
        let init = line
            .checked_sub(1)
            .map_or(0, |prev| self.source.inner.newlines.index(prev) + 1);
        let end = self
            .source
            .inner
            .newlines
            .get(line + 1)
            .map_or(self.source.len(), |next| next + 1);
        Span::new(Self::new(self.source.clone(), init), end - init)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let (line, column) = self.line_column();
        fmtr.debug_struct("Location")
            .field("source", &self.source)
            .field("position", &self.position)
            .field("line", &line)
            .field("column", &column)
            .finish()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let (line, column) = self.line_column();
        write!(fmtr, "in {} ({}, {})", self.source, line + 1, column + 1)
    }
}
