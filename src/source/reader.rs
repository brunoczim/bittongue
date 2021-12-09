//! This module provides means of navigating on a source code, a stream.

use super::{Location, Source, Span};

/// A reader of a source code, a stream.
///
/// See [`Source::reader`](Source::reader) to create a reader.
#[derive(Debug, Clone)]
pub struct Reader {
    /// The source code this reader is reading.
    source: Source,
    /// The position (in string segments) in the source this reader is.
    position: usize,
    /// Last position (in string segments) marked by the reader.
    marked: usize,
}

impl Reader {
    /// Creates a new reader given the source code object it will read.
    pub(super) fn new(source: Source) -> Self {
        Self { source, position: 0, marked: 0 }
    }

    /// Is the end of file reached?
    pub fn is_eof(&self) -> bool {
        self.current().is_none()
    }

    /// Position in string segments that the reader is currently at.
    pub fn position(&self) -> usize {
        self.position
    }

    /// The current string segment rendered.
    pub fn current(&self) -> Option<&str> {
        self.source.get(self.position)
    }

    /// A string segment from current position until `current + additional`,
    /// rendered.
    pub fn current_to(&self, additional: usize) -> Option<&str> {
        self.source.get(self.position .. self.position + additional)
    }

    /// The marked position (in string segments).
    pub fn marked(&self) -> usize {
        self.marked
    }

    /// The source code this reader is reading.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Location at the given position.
    pub fn location(&self) -> Location {
        Location::new(self.source.clone(), self.position)
    }

    /// [`Span`](Span) from the marked position up to the current
    /// position.
    pub fn span(&self) -> Span {
        if self.marked < self.position {
            let loc = Location::new(self.source.clone(), self.marked);
            Span::new(loc, self.position - self.marked)
        } else {
            Span::new(self.location(), self.marked - self.position)
        }
    }

    /// Marks the current position so it can be used to create a
    /// [`Span`](Span) later.
    pub fn mark(&mut self) {
        self.marked = self.position;
    }

    /// Advances the stream by 1 and returns whether it did move.
    pub fn next(&mut self) -> bool {
        self.advance(1) == 1
    }

    /// Goes back on the stream by 1 and returns whether it did move.
    pub fn prev(&mut self) -> bool {
        self.rollback(1) == 1
    }

    /// Advance the stream by the given `count` of string segments, and return
    /// how much it actually moved.
    pub fn advance(&mut self, count: usize) -> usize {
        let advanced = count.min(self.source.len() - self.position);
        self.position += advanced;
        advanced
    }

    /// Goes back on the stream by the given `count` of string segments, and
    /// return how much it actually moved.
    pub fn rollback(&mut self, count: usize) -> usize {
        let rolled = count.min(self.position);
        self.position -= rolled;
        rolled
    }

    pub fn expect(&mut self, mut expected: &str) -> bool {
        let mut count = 0;

        while expected.len() > 0 {
            if let Some(found) =
                self.current().filter(|found| expected.starts_with(found))
            {
                expected = &expected[found.len() ..];
                count += 1;
                self.next();
            } else {
                self.rollback(count);
                return false;
            }
        }

        true
    }
}
