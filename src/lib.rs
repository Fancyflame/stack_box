#![no_std]

mod container;
mod macros;
mod utils;

use core::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::{drop_in_place, null_mut},
};

pub use container::{CalculateContainer, StackBoxContainer};
use utils::{with_metadata, with_metadata_mut};

pub struct StackBox<T, Ctnr>
where
    T: ?Sized,
{
    container: MaybeUninit<Ctnr>,
    reinterpreter: *const T,
}

impl<T, Ctnr> StackBox<T, Ctnr>
where
    Ctnr: StackBoxContainer,
{
    pub fn new(value: T) -> Option<Self> {
        if align_of::<T>() > align_of::<Ctnr>() || size_of::<T>() > size_of::<Ctnr>() {
            None
        } else {
            Some(unsafe { Self::new_unchecked(value) })
        }
    }

    pub unsafe fn new_unchecked(value: T) -> Self {
        let mut container: MaybeUninit<Ctnr> = MaybeUninit::uninit();
        (container.as_mut_ptr() as *mut T).write(value);

        Self {
            container,
            reinterpreter: null_mut(),
        }
    }

    pub fn coerce_unsized<U, F>(this: Self, check_fn: F) -> StackBox<U, Ctnr>
    where
        U: ?Sized,
        F: Fn(&mut UnsizeChecker<U, T>),
    {
        let mut this = ManuallyDrop::new(this);

        let value: T = unsafe { this.container.as_mut_ptr().cast::<T>().read() };
        let mut checker = UnsizeChecker {
            ptr: None,
            src: value,
        };

        // even if this function panics, the value could properly drop
        check_fn(&mut checker);

        let Some(reinterpreter) = checker.ptr else {
            panic!("checking of the unsize checker is not passed");
        };

        let mut container: MaybeUninit<Ctnr> = MaybeUninit::uninit();
        unsafe {
            container.as_mut_ptr().cast::<T>().write(checker.src);
        }

        StackBox {
            container,
            reinterpreter,
        }
    }
}

impl<T, Ctnr> StackBox<T, Ctnr>
where
    T: ?Sized,
{
    pub fn as_ptr(this: &mut Self) -> *mut T {
        with_metadata_mut(this.container.as_mut_ptr().cast(), this.reinterpreter)
    }
}

pub struct UnsizeChecker<Target, Src = Target>
where
    Target: ?Sized,
    Src: ?Sized,
{
    ptr: Option<*mut Target>,
    src: Src,
}

impl<T> UnsizeChecker<T, T>
where
    T: ?Sized,
{
    pub fn set_checked(&mut self) {
        self.ptr = Some(&raw mut self.src);
    }
}

impl<T, Ctnr> Deref for StackBox<T, Ctnr>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*with_metadata(self.container.as_ptr().cast(), self.reinterpreter) }
    }
}

impl<T, Ctnr> DerefMut for StackBox<T, Ctnr>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *Self::as_ptr(self) }
    }
}

impl<T, Ctnr> Drop for StackBox<T, Ctnr>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            drop_in_place(Self::as_ptr(self));
        }
    }
}

impl<T, Ctnr> Clone for StackBox<T, Ctnr>
where
    T: Clone,
    Ctnr: StackBoxContainer,
{
    fn clone(&self) -> Self {
        unsafe { Self::new_unchecked((**self).clone()) }
    }
}

impl<T, U, Ctnr, Ctnr2> PartialEq<StackBox<U, Ctnr2>> for StackBox<T, Ctnr>
where
    T: PartialEq<U> + ?Sized,
    U: ?Sized,
{
    fn eq(&self, other: &StackBox<U, Ctnr2>) -> bool {
        **self == **other
    }
}

impl<T, Ctnr> Eq for StackBox<T, Ctnr> where T: Eq + ?Sized {}

impl<T, U, Ctnr, Ctnr2> PartialOrd<StackBox<U, Ctnr2>> for StackBox<T, Ctnr>
where
    T: PartialOrd<U> + ?Sized,
    U: ?Sized,
{
    fn partial_cmp(&self, other: &StackBox<U, Ctnr2>) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T, Ctnr> Ord for StackBox<T, Ctnr>
where
    T: Ord + ?Sized,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}
