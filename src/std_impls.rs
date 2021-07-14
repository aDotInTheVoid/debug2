use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ops::Deref;

use crate::{Debug2, Formatter2, Result};

macro_rules! std_debug {
    ($($t:ty),+) => {
        $(
            impl Debug2 for $t {
                fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
                    f.write_debug(self)
                }
            }
        )+
    };
}

std_debug! {
    String, &str, bool, (),
    i8, i16, i32, i64, i128, isize,
    f32, f64
}

impl<T: ?Sized + Debug2> Debug2 for &T {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        Debug2::fmt2(&**self, f)
    }
}

impl<T: ?Sized + Debug2> Debug2 for &mut T {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        Debug2::fmt2(&**self, f)
    }
}

impl<T: Debug2> Debug2 for [T] {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// TODO: Macro these
impl<T: Debug2> Debug2 for Vec<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T: Debug2> Debug2 for VecDeque<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T: Debug2> Debug2 for LinkedList<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T: Debug2> Debug2 for BinaryHeap<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
impl<K, V, S> Debug2 for HashMap<K, V, S>
where
    K: Debug2,
    V: Debug2,
{
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<K: Debug2, V: Debug2> Debug2 for BTreeMap<K, V> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<T, S> Debug2 for HashSet<T, S>
where
    T: Debug2,
{
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
impl<T> Debug2 for BTreeSet<T>
where
    T: Debug2,
{
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
impl<T: Debug2> Debug2 for Option<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        match self {
            Some(v) => f.debug_tuple("Some").field(v).finish(),
            None => f.debug_tuple("None").finish(),
        }
    }
}

// Tuple

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:Debug2),+> Debug2 for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
                let mut builder = f.debug_tuple("");
                let ($(ref $name,)+) = *self;
                $(
                    builder.field(&$name);
                )+

                builder.finish()
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

impl<T: ?Sized> Debug2 for std::marker::PhantomData<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_struct("PhantomData").finish()
    }
}

impl<T: Copy + Debug2> Debug2 for std::cell::Cell<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_struct("Cell").field("value", &self.get()).finish()
    }
}

impl<T: ?Sized + Debug2> Debug2 for std::cell::RefCell<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        match self.try_borrow() {
            Ok(borrow) => f.debug_struct("RefCell").field("value", &borrow).finish(),
            Err(_) => {
                // The RefCell is mutably borrowed so we can't look at its value
                // here. Show a placeholder instead.
                struct BorrowedPlaceholder;

                impl Debug2 for BorrowedPlaceholder {
                    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
                        f.write_str("<borrowed>")
                    }
                }

                f.debug_struct("RefCell")
                    .field("value", &BorrowedPlaceholder)
                    .finish()
            }
        }
    }
}

impl<T: ?Sized + Debug2> Debug2 for std::cell::Ref<'_, T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        Debug2::fmt2(&**self, f)
    }
}

impl<T: ?Sized + Debug2> Debug2 for std::cell::RefMut<'_, T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        Debug2::fmt2(&*(self.deref()), f)
    }
}

impl<T: ?Sized> Debug2 for std::cell::UnsafeCell<T> {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result {
        f.debug_struct("UnsafeCell").finish_non_exhaustive()
    }
}
