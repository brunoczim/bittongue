use crate::source::Span;
use std::{
    any::{Any, TypeId},
    fmt,
    rc::Rc,
    sync::Arc,
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Note,
    Warning,
    Error,
}

pub trait Diagnostic: fmt::Display + fmt::Debug + Any {
    fn level(&self) -> Level;

    fn primary_span(&self) -> Option<Span>;

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

#[derive(Debug)]
pub struct Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    elements: Vec<Box<D>>,
}

impl<D> Diagnostics<D>
where
    D: Diagnostic + ?Sized,
{
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn raise<T>(&mut self, diagnostic: T)
    where
        Box<T>: Into<Box<D>>,
    {
        self.elements.push(Box::new(diagnostic).into());
    }
}

#[derive(Debug)]
pub struct IntoIter<D>
where
    D: Diagnostic + ?Sized,
{
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
