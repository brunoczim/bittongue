//! This module exports utilites related to grapheme clusters/unicode
//! segmentation.
//!
//! Some "visual" characteres are actually multiple unicode characters, such as
//! letter "a" with three diacritics "ā̤́", those are grapheme clusters.

use std::{cmp::Ordering, fmt, mem, rc::Rc, sync::Arc};
use unicode_segmentation::UnicodeSegmentation;

/// A single grapheme cluster. Unsized type.
#[repr(transparent)]
#[derive(Debug, Eq, Ord, Hash)]
pub struct GraphemeCluster {
    /// Contents of the cluster.
    content: str,
}

impl<S> PartialEq<S> for GraphemeCluster
where
    S: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &S) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<GraphemeCluster> for str {
    fn eq(&self, other: &GraphemeCluster) -> bool {
        self == other.as_str()
    }
}

impl<S> PartialOrd<S> for GraphemeCluster
where
    S: AsRef<str> + ?Sized,
{
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<GraphemeCluster> for str {
    fn partial_cmp(&self, other: &GraphemeCluster) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl AsRef<str> for GraphemeCluster {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'grapheme> From<&'grapheme GraphemeCluster> for Box<GraphemeCluster> {
    fn from(grapheme: &'grapheme GraphemeCluster) -> Self {
        unsafe { mem::transmute(Box::<str>::from(grapheme.as_str())) }
    }
}

impl<'grapheme> From<&'grapheme GraphemeCluster> for Rc<GraphemeCluster> {
    fn from(grapheme: &'grapheme GraphemeCluster) -> Self {
        unsafe { mem::transmute(Rc::<str>::from(grapheme.as_str())) }
    }
}

impl<'grapheme> From<&'grapheme GraphemeCluster> for Arc<GraphemeCluster> {
    fn from(grapheme: &'grapheme GraphemeCluster) -> Self {
        unsafe { mem::transmute(Arc::<str>::from(grapheme.as_str())) }
    }
}

impl GraphemeCluster {
    /// Only call this if you know `content` is a single grapheme cluster.
    pub(crate) fn new_unchecked(content: &str) -> &Self {
        unsafe { mem::transmute(content) }
    }

    /// Creates a new grapheme cluster reference from a string borrowed
    /// reference. The given string must be exactly a single grapheme cluster,
    /// if not, `None` is returned.
    pub fn new(content: &str) -> Option<&Self> {
        let mut iter = content.graphemes(true);
        if iter.next().is_some() && iter.next().is_none() {
            Some(Self::new_unchecked(content))
        } else {
            None
        }
    }

    /// Display this grapheme cluster as a string.
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Length in bytes of this grapheme cluster.
    pub fn byte_len(&self) -> usize {
        self.as_str().len()
    }

    /// Counts how many unicode characters compose this grapheme cluster.
    pub fn count_chars(&self) -> usize {
        self.as_str().chars().count()
    }

    /// Returns whether this grapheme cluster has diacritics, i.e. additional
    /// characters.
    pub fn has_diacritics(&self) -> bool {
        let mut iter = self.as_str().chars();
        iter.next();
        iter.next().is_some()
    }

    /// Returns the first character of this grapheme cluster, stripping
    /// diacritics.
    pub fn strip_diacritics(&self) -> char {
        self.as_str().chars().next().unwrap()
    }

    /// Returns whether this grapheme cluster is a single character.
    pub fn is_char(&self) -> bool {
        self.to_char().is_some()
    }

    /// Attempts to convert this grapheme cluster to a single character. Only
    /// succeeds if the cluster is composed of a single character.
    pub fn to_char(&self) -> Option<char> {
        let mut iter = self.as_str().chars();
        iter.next().filter(|_| iter.next().is_none())
    }

    /// Returns whether this grapheme cluster is alphabetic (possibly with
    /// diacritics).
    pub fn is_alphabetic(&self) -> bool {
        self.strip_diacritics().is_whitespace()
    }

    /// Returns whether this grapheme cluster is alphabetic (and only if no
    /// diacritics are present).
    pub fn is_alphabetic_char(&self) -> bool {
        self.to_char().map_or(false, char::is_alphabetic)
    }

    /// Returns whether this grapheme cluster is a single ASCII alphabetic
    /// character.
    pub fn is_ascii_alphabetic(&self) -> bool {
        self.to_char().map_or(false, |ch| ch.is_ascii_alphabetic())
    }

    /// Returns whether this grapheme cluster is numeric (possibly with
    /// diacritics).
    pub fn is_numeric(&self) -> bool {
        self.strip_diacritics().is_numeric()
    }

    /// Returns whether this grapheme cluster is numeric (and only if no
    /// diacritics are present).
    pub fn is_numeric_char(&self) -> bool {
        self.to_char().map_or(false, char::is_numeric)
    }

    /// Returns whether this grapheme cluster is a single ASCII numeric
    /// character.
    pub fn is_ascii_numeric(&self) -> bool {
        self.to_char().map_or(false, |ch| ch.is_ascii_digit())
    }

    /// Returns whether this grapheme cluster is alphabetic or numeric (possibly
    /// with diacritics).
    pub fn is_alphanumeric(&self) -> bool {
        self.strip_diacritics().is_alphanumeric()
    }

    /// Returns whether this grapheme cluster is alphabetic or numeric (and only
    /// if no diacritics are present).
    pub fn is_alphanumeric_char(&self) -> bool {
        self.to_char().map_or(false, char::is_alphanumeric)
    }

    /// Returns whether this grapheme cluster is a single ASCII alphabetic or
    /// numeric character.
    pub fn is_ascii_alphanumeric(&self) -> bool {
        self.to_char().map_or(false, |ch| ch.is_ascii_alphanumeric())
    }

    /// Returns whether this grapheme cluster is a digit of the given base.
    /// Only base 2 to 36 are allowed, using ASCII alphanumeric characters.
    /// Diacritics are allowed but ignored.
    pub fn is_digit(&self, base: u32) -> bool {
        self.strip_diacritics().is_digit(base)
    }

    /// Returns whether this grapheme cluster is a digit of the given base.
    /// Only base 2 to 36 are allowed, using ASCII alphanumeric characters.
    /// Diacritics are not allowed and yield `false`.
    pub fn is_digit_char(&self, base: u32) -> bool {
        self.to_char().map_or(false, |ch| ch.is_digit(base))
    }

    /// Attempts to convert this grapheme cluster into a digit of the given
    /// base. Only base 2 to 36 are allowed, using ASCII alphanumeric
    /// characters. Diacritics are allowed but ignored.
    pub fn to_digit(&self, base: u32) -> Option<u32> {
        self.strip_diacritics().to_digit(base)
    }

    /// Attempts to convert this grapheme cluster into a digit of the given
    /// base. Only base 2 to 36 are allowed, using ASCII alphanumeric
    /// characters. Diacritics are not allowed and yield `None`.
    pub fn char_to_digit(&self, base: u32) -> Option<u32> {
        self.to_char().and_then(|ch| ch.to_digit(base))
    }

    /// Returns whether this grapheme cluster is whitespace (possibly with
    /// diacritics).
    pub fn is_whitespace(&self) -> bool {
        self.strip_diacritics().is_whitespace()
    }

    /// Returns whether this grapheme cluster is whitespace (and only if no
    /// diacritics are present).
    pub fn is_whitespace_char(&self) -> bool {
        self.to_char().map_or(false, char::is_whitespace)
    }
}

impl fmt::Display for GraphemeCluster {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmtr)
    }
}

/// Creates an iterator over grapheme clusters of a string.
pub fn iter(input: &str) -> Iter {
    Iter { inner: input.graphemes(true) }
}

/// Creates an indexed iterator over grapheme clusters of a string, yielding
/// grapheme indices in terms of bytes together with the graphemes.
pub fn indexed_iter(input: &str) -> IndexedIter {
    IndexedIter { inner: input.grapheme_indices(true) }
}

/// An iterator over grapheme clusters of a string.
#[derive(Debug, Clone)]
pub struct Iter<'input> {
    inner: unicode_segmentation::Graphemes<'input>,
}

impl<'input> Iterator for Iter<'input> {
    type Item = &'input GraphemeCluster;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(GraphemeCluster::new_unchecked)
    }
}

impl<'input> DoubleEndedIterator for Iter<'input> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(GraphemeCluster::new_unchecked)
    }
}

/// An indexed iterator over grapheme clusters of a string, yielding
/// grapheme indices in terms of bytes together with the graphemes.
#[derive(Clone)]
pub struct IndexedIter<'input> {
    inner: unicode_segmentation::GraphemeIndices<'input>,
}

impl<'input> fmt::Debug for IndexedIter<'input> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("IndexedIter").finish()
    }
}

impl<'input> Iterator for IndexedIter<'input> {
    type Item = (usize, &'input GraphemeCluster);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(index, content)| {
            (index, GraphemeCluster::new_unchecked(content))
        })
    }
}

impl<'input> DoubleEndedIterator for IndexedIter<'input> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(index, content)| {
            (index, GraphemeCluster::new_unchecked(content))
        })
    }
}
