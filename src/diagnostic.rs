//! This module exports error-related utilities for parse/compile-time of a
//! programming language.

use crate::source::Span;
use std::{
    any::{Any, TypeId},
    fmt,
    rc::Rc,
    slice,
    sync::Arc,
    vec,
};

/// Level of a given diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// This is just a note, easily ignored.
    Note,
    /// This is a warning, should be read carefully.
    Warning,
    /// A hard error, cannot be ignored at all.
    Error,
}

/// Diagnostic: a problem or note found in a source code.
pub trait Diagnostic: fmt::Display + fmt::Debug + Any {
    /// Severity level of this diagnostic.
    fn level(&self) -> Level;

    /// Yields the primary span related to the diagnostic, if any.
    fn primary_span(&self) -> Option<Span>;

    /// Yields an iterator over secondary spans related to the diagnostic, if
    /// any.
    fn secondary_spans<'this>(
        &'this self,
    ) -> Option<Box<dyn Iterator<Item = Span> + Send + Sync + 'this>> {
        None
    }
}

macro_rules! impl_for_ptr {
    (<($($generics:tt)*)> $ty:ty where $($bounds:tt)*) => {
        impl<$($generics)*> Diagnostic for $ty
        where
            $($bounds)*
        {
            fn level(&self) -> Level {
                (**self).level()
            }

            fn primary_span(&self) -> Option<Span> {
                (**self).primary_span()
            }

            fn secondary_spans<'this>(
                &'this self,
            ) -> Option<Box<dyn Iterator<Item = Span> + Send + Sync + 'this>> {
                (**self).secondary_spans()
            }
        }
    };
}

impl_for_ptr! { <(T)> &'static T where T: Diagnostic + ?Sized }
impl_for_ptr! { <(T)> &'static mut T where T: Diagnostic + ?Sized }
impl_for_ptr! { <(T)> Box<T> where T: Diagnostic + ?Sized }
impl_for_ptr! { <(T)> Rc<T> where T: Diagnostic + ?Sized }
impl_for_ptr! { <(T)> Arc<T> where T: Diagnostic + ?Sized }

impl<'diag, T> From<Box<T>> for Box<dyn Diagnostic + 'diag>
where
    T: Diagnostic + 'diag,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<T> From<Box<T>> for Box<dyn Diagnostic + Send>
where
    T: Diagnostic + Send,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<T> From<Box<T>> for Box<dyn Diagnostic + Sync>
where
    T: Diagnostic + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<T> From<Box<T>> for Box<dyn Diagnostic + Send + Sync>
where
    T: Diagnostic + Send + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

macro_rules! impl_downcast {
    ($ty:ty) => {
        impl $ty {
            pub fn is<T>(&self) -> bool
            where
                T: Diagnostic,
            {
                Any::type_id(self) == TypeId::of::<Self>()
            }

            pub fn downcast<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>
            where
                T: Diagnostic,
            {
                if self.is::<T>() {
                    let raw = Box::into_raw(self);
                    Ok(unsafe { Box::from_raw(raw as *mut T) })
                } else {
                    Err(self)
                }
            }

            pub fn downcast_ref<T>(&self) -> Option<&T>
            where
                T: Diagnostic,
            {
                if self.is::<T>() {
                    Some(unsafe {
                        &*(self as *const dyn Diagnostic as *const T)
                    })
                } else {
                    None
                }
            }

            pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
            where
                T: Diagnostic,
            {
                if self.is::<T>() {
                    Some(unsafe {
                        &mut *(self as *mut dyn Diagnostic as *mut T)
                    })
                } else {
                    None
                }
            }
        }
    };
}

impl_downcast! { dyn Diagnostic }
impl_downcast! { dyn Diagnostic + Send }
impl_downcast! { dyn Diagnostic + Sync }
impl_downcast! { dyn Diagnostic + Send + Sync }

/// A collection of diagnostics. Generic on the type of diagnostics, but
/// intended for trait objects, such as `dyn Diagnostic` or `dyn Diagnostic +
/// Send + Sync` (the last one is the default).
#[derive(Debug)]
pub struct Diagnostics<D = dyn Diagnostic + Send + Sync>
where
    D: Diagnostic + ?Sized,
{
    elements: Vec<Box<D>>,
}

impl<D> Default for Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    /// Creates an empty collection of diagnostics.
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    /// Returns whether the source code status is OK (no hard errors) according
    /// to this collection of diagnostics.
    pub fn is_ok(&self) -> bool {
        self.max_level().map_or(true, |level| level < Level::Error)
    }

    /// Returns whether the source code status is NOT OK (there are hard errors)
    /// according to this collection of diagnostics.
    pub fn is_err(&self) -> bool {
        self.max_level().map_or(false, |level| level >= Level::Error)
    }

    /// Returns the maximum level among the diagnostics in this collection.
    pub fn max_level(&self) -> Option<Level> {
        self.iter().map(Diagnostic::level).max()
    }

    /// Creates an iterator over references of diagnostics.
    pub fn iter(&self) -> Iter<D> {
        self.into_iter()
    }

    /// Raises a new diagnostic and saves it in this collection.
    pub fn raise<T>(&mut self, diagnostic: T)
    where
        Box<T>: Into<Box<D>>,
    {
        self.elements.push(Box::new(diagnostic).into());
    }
}

/// Owned iterator over diagnostics of the [`Diagnostics`] collection.
#[derive(Debug)]
pub struct IntoIter<D>
where
    D: Diagnostic + ?Sized,
{
    /// Inner owned vector iterator.
    inner: vec::IntoIter<Box<D>>,
}

impl<D> Iterator for IntoIter<D>
where
    D: Diagnostic + ?Sized,
{
    type Item = Box<D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<D> IntoIterator for Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    type Item = Box<D>;
    type IntoIter = IntoIter<D>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self.elements.into_iter() }
    }
}

/// Borrowed iterator over diagnostics of the [`Diagnostics`] collection.
#[derive(Debug)]
pub struct Iter<'diag, D>
where
    D: Diagnostic + ?Sized,
{
    /// Iterator over slice references.
    inner: slice::Iter<'diag, Box<D>>,
}

impl<'diag, D> Iterator for Iter<'diag, D>
where
    D: Diagnostic + ?Sized,
{
    type Item = &'diag D;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|boxed| boxed.as_ref())
    }
}

impl<'diag, D> IntoIterator for &'diag Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    type Item = &'diag D;
    type IntoIter = Iter<'diag, D>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { inner: self.elements.iter() }
    }
}
