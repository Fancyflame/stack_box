use core::ops::{Deref, DerefMut};

use crate::{StackBox, StackBoxContainer};

/// Simple wrapper to choose heap box and stack box automatically
pub enum AutoBox<T, Ctnr>
where
    T: ?Sized,
{
    Stack(StackBox<T, Ctnr>),
    Heap(Box<T>),
}

impl<T, Ctnr> AutoBox<T, Ctnr>
where
    Ctnr: StackBoxContainer,
{
    pub fn new(value: T) -> Self {
        match StackBox::<_, Ctnr>::new_dynamic(value) {
            Ok(sb) => Self::Stack(sb),
            Err((value, _)) => Self::Heap(Box::new(value)),
        }
    }

    pub const fn is_stack(this: &Self) -> bool {
        matches!(this, Self::Stack(_))
    }

    pub const fn is_heap(this: &Self) -> bool {
        matches!(this, Self::Heap(_))
    }
}

impl<T: ?Sized, Ctnr> Deref for AutoBox<T, Ctnr> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Stack(b) => &**b,
            Self::Heap(b) => &**b,
        }
    }
}

impl<T: ?Sized, Ctnr> DerefMut for AutoBox<T, Ctnr> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Stack(b) => &mut **b,
            Self::Heap(b) => &mut **b,
        }
    }
}
