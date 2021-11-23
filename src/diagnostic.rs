use crate::source::Span;
use std::{
    any::{Any, TypeId},
    fmt,
    marker::PhantomData,
    rc::Rc,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Note,
    Warning,
    Error,
}

pub trait Diagnostic: fmt::Display + fmt::Debug {
    fn level(&self) -> Level;

    fn primary_span(&self) -> Option<Span>;

    fn secondary_span<'this>(
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

            fn secondary_span<'this>(
                &'this self,
            ) -> Option<Box<dyn Iterator<Item = Span> + Send + Sync + 'this>> {
                (**self).secondary_span()
            }
        }
    };
}

impl_for_ptr! { <('diag, T)> &'diag T where T: Diagnostic + ?Sized }
impl_for_ptr! { <('diag, T)> &'diag mut T where T: Diagnostic + ?Sized }
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

impl<'diag, T> From<Box<T>> for Box<dyn Diagnostic + Send + 'diag>
where
    T: Diagnostic + 'diag + Send,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<'diag, T> From<Box<T>> for Box<dyn Diagnostic + Sync + 'diag>
where
    T: Diagnostic + 'diag + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<'diag, T> From<Box<T>> for Box<dyn Diagnostic + Send + Sync + 'diag>
where
    T: Diagnostic + 'diag + Send + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

pub trait AnyDiagnostic: Diagnostic + Any {}

impl<T> AnyDiagnostic for T where T: Any + Diagnostic + ?Sized {}

macro_rules! impl_downcast {
    ($ty:ty) => {
        impl $ty {
            pub fn is<T>(&self) -> bool
            where
                T: Diagnostic + 'static,
            {
                Any::type_id(self) == TypeId::of::<Self>()
            }

            pub fn downcast<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>
            where
                T: Diagnostic + 'static,
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
                T: Diagnostic + 'static,
            {
                if self.is::<T>() {
                    Some(unsafe {
                        &*(self as *const dyn AnyDiagnostic as *const T)
                    })
                } else {
                    None
                }
            }

            pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
            where
                T: Diagnostic + 'static,
            {
                if self.is::<T>() {
                    Some(unsafe {
                        &mut *(self as *mut dyn AnyDiagnostic as *mut T)
                    })
                } else {
                    None
                }
            }
        }
    };
}

impl_downcast! { dyn AnyDiagnostic }
impl_downcast! { dyn AnyDiagnostic + Send }
impl_downcast! { dyn AnyDiagnostic + Sync }
impl_downcast! { dyn AnyDiagnostic + Send + Sync }

impl<'diag, T> From<Box<T>> for Box<dyn AnyDiagnostic + 'diag>
where
    T: AnyDiagnostic + 'diag,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<'diag, T> From<Box<T>> for Box<dyn AnyDiagnostic + Send + 'diag>
where
    T: AnyDiagnostic + 'diag + Send,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<'diag, T> From<Box<T>> for Box<dyn AnyDiagnostic + Sync + 'diag>
where
    T: AnyDiagnostic + 'diag + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

impl<'diag, T> From<Box<T>> for Box<dyn AnyDiagnostic + Send + Sync + 'diag>
where
    T: AnyDiagnostic + 'diag + Send + Sync,
{
    fn from(implementor: Box<T>) -> Self {
        implementor
    }
}

#[derive(Debug)]
pub struct Diagnostics<'diag, T = dyn Diagnostic + Send + Sync + 'diag>
where
    T: Diagnostic + 'diag,
{
    elements: Vec<Box<T>>,
    _marker: PhantomData<dyn Diagnostic + 'diag>,
}

impl<'diag, T> Diagnostics<'diag, T>
where
    T: Diagnostic + 'diag,
{
    pub fn new() -> Self {
        Self { elements: Vec::new(), _marker: PhantomData }
    }

    pub fn raise<D>(&mut self, diagnostic: D)
    where
        Box<D>: Into<Box<T>>,
    {
        self.elements.push(Box::new(diagnostic).into())
    }
}

pub type AnyDiagnostics = Diagnostics<'static, dyn AnyDiagnostic + Send + Sync>;
