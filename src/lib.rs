#![warn(missing_docs)]

//! `debug2` is a pretty printing crate based on [`std::fmt`]
//!
//! # Why not just use [`Debug`]
//!
//! The [`Debug`] trait is good, but the problem is it is not very good at nested stuctures.
//! Either you use `{:?}` and get a line that is too long, or too many lines with not enough
//! information on them.
//!
//! ```rust
//! let complex_structure = vec![
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//! ];
//!
//! let one_line = format!("{:?}", complex_structure);
//!
//! assert_eq!(one_line, "[[Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)], [Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)], [Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)]]");
//!
//! let many_lines = format!("{:#?}", complex_structure);
//!
//! assert_eq!(many_lines, "[
//!     [
//!         Some(
//!             1,
//!         ),
//!         Some(
//!             2,
//!         ),
//!         Some(
//!             3,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             2,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             4,
//!         ),
//!         Some(
//!             7,
//!         ),
//!     ],
//!     [
//!         Some(
//!             1,
//!         ),
//!         Some(
//!             2,
//!         ),
//!         Some(
//!             3,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             2,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             4,
//!         ),
//!         Some(
//!             7,
//!         ),
//!     ],
//!     [
//!         Some(
//!             1,
//!         ),
//!         Some(
//!             2,
//!         ),
//!         Some(
//!             3,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             2,
//!         ),
//!         None,
//!     ],
//!     [
//!         Some(
//!             4,
//!         ),
//!         Some(
//!             7,
//!         ),
//!     ],
//! ]")
//! ```
//!
//! `pprint` aims to be a third alternative, that gets this correct.
//!
//! ```rust
//! use debug2::pprint;
//!
//! let complex_structure = vec![
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//!     vec![Some(1), Some(2), Some(3), None],
//!     vec![Some(2), None],
//!     vec![Some(4), Some(7)],
//! ];
//!
//! assert_eq!(
//!     pprint(complex_structure),
//!     "\
//! [
//!     [Some(1), Some(2), Some(3), None],
//!     [Some(2), None],
//!     [Some(4), Some(7)],
//!     [Some(1), Some(2), Some(3), None],
//!     [Some(2), None],
//!     [Some(4), Some(7)],
//!     [Some(1), Some(2), Some(3), None],
//!     [Some(2), None],
//!     [Some(4), Some(7)],
//! ]"
//! );
//! ```
//!
//! To use, derive [`Debug`] for your types, and then use [`pprint`] to print them.
//!
//! You can also manually implement [`Debug`], using a subset of the API in [`std::fmt::Formatter`]
//!
//! # Limitations
//! - Speed: While doing this will always mean extra work, this crate is paticularly inefficient.
//! - Prevalence: Almost every type implements [`std::fmt::Debug`], but not this type
//! - The derive isn't great: The derive macro for [`std::fmt::Debug`] works everywhere. This one
//!   is kind of basic, and will probably not work everywhere it should.

use std::fmt::{Debug as StdDebug, Error, Result, Write};

mod builders;
mod std_impls;

pub use builders::{DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple};

pub use derive::*;

const MAX_LEN: usize = 80;
/// Pretty Printed Formatting
///
/// This is much like [`std::fmt::Debug`], but it supports much better multiline output
///
/// # Examples
///
/// ```rust
/// use debug2::{pprint, Debug};
///
/// #[derive(Debug)]
/// struct Numbers {
///     a: Vec<Vec<i32>>,
///     b: String,
/// }
///
/// let a = Numbers {
///     a: vec![vec![10; 10]; 2],
///     b: "FooBar".to_owned(),
/// };
///
/// assert_eq!(
///     pprint(&a),
///     "\
/// Numbers {
///     a: [
///         [10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
///         [10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
///     ],
///     b: \"FooBar\",
/// }"
/// );
/// ```
///
/// You can also implement `fmt` manually, using an API much like [`std::fmt::Formatter`]
///
/// ```rust
/// use debug2::{pprint, Debug, Formatter};
/// use std::fmt;
///
/// struct Chunked10([u8; 100]);
///
/// impl Debug for Chunked10 {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         f.debug_list().entries(self.0.chunks(10)).finish()
///     }
/// }
///
/// assert_eq!(
///     pprint(Chunked10([0; 100])),
///     "\
/// [
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
///     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
/// ]"
/// );
/// ```
pub trait Debug {
    /// Formats the value using the given formatter.
    ///
    /// Note that this may be called more than once for any invocation of `pprint`, if you do
    /// side effects in this, make sure they are idempotent. In general, don't relly on how often
    /// this function is called, as it may change in a future release.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

/// Configuration for formatting.
///
/// A `Formatter` represents various options related to formatting. Users do not
/// construct `Formatter`s directly; a mutable reference to one is passed to
/// the `fmt` method of [`Debug`].
///
/// To interact with a `Formatter`, you'll call various methods to change the
/// various options related to formatting. For examples, please see the
/// documentation of the methods defined on `Formatter` below.
pub struct Formatter<'a> {
    buf: &'a mut (dyn Write + 'a),
    mode: Mode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Pretty,
    Flat,
}

fn flatprint_checked<T: Debug>(x: T) -> std::result::Result<String, Error> {
    pprint_mode(x, Mode::Flat)
}

fn pprint_mode<T: Debug>(x: T, mode: Mode) -> std::result::Result<String, Error> {
    let mut out = String::new();
    let mut f = Formatter {
        buf: &mut out,
        mode,
    };
    x.fmt(&mut f)?;
    Ok(out)
}

/// Pretty Print an item to a string, or return an error
///
/// ```rust
/// use debug2::{pprint_checked, Debug, Formatter};
/// use std::fmt;
///
/// struct Good;
/// struct Bad;
///
/// impl Debug for Good {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         f.debug_struct("Good").finish()
///     }
/// }
///
/// impl Debug for Bad {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         Err(fmt::Error)
///     }
/// }
///
/// assert!(pprint_checked(Good).is_ok());
/// assert!(pprint_checked(Bad).is_err());
/// ```
pub fn pprint_checked<T: Debug>(x: T) -> std::result::Result<String, Error> {
    let flat = flatprint_checked(&x)?;
    if flat.len() <= MAX_LEN {
        Ok(flat)
    } else {
        pprint_mode(x, Mode::Pretty)
    }
}

/// Pretty Print an item to a string
///
/// ```rust
/// use debug2::pprint;
///
/// let x: Vec<Option<&[i32]>> = vec![Some(&[1; 20]), None, None, Some(&[1, 2, 3])];
///
/// assert_eq!(
///     pprint(x),
///     "\
/// [
///     Some([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
///     None,
///     None,
///     Some([1, 2, 3]),
/// ]"
/// );
/// ```
///
/// Note that while this takes a `T`, you can also pass a reference due to the
/// `impl<T: Debug> Debug for `&T`
///
/// # Panics
///
/// This will panic if `<T as Debug>::fmt` returns an error
///
pub fn pprint<T: Debug>(x: T) -> String {
    pprint_checked(x).unwrap()
}

impl<'a> Formatter<'a> {
    fn write_debug<T: StdDebug>(&mut self, val: &T) -> Result {
        write!(self.buf, "{:?}", val)
    }

    fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    /// Creates a [`DebugStruct`] builder designed to assist with creation of
    /// [`Debug`] implementations for structs.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debug2::{pprint, Debug, Formatter};
    /// use std::fmt;
    /// use std::net::Ipv4Addr;
    ///
    /// struct Foo {
    ///     bar: i32,
    ///     baz: String,
    ///     addr: Ipv4Addr,
    /// }
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    ///         fmt.debug_struct("Foo")
    ///             .field("bar", &self.bar)
    ///             .field("baz", &self.baz)
    /// # // TODO: The viersion in `std` uses `format_args`, which gives
    /// # // a different result, because it doesn't have "" around the content
    /// # // I should have a macro that delegated to `format!`, but returns a
    /// # // newtype that doesnt add quotes
    ///             .field("addr", &format!("{}", self.addr))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo { bar: 10, baz: \"Hello World\", addr: \"127.0.0.1\" }",
    ///     pprint(Foo {
    ///         bar: 10,
    ///         baz: "Hello World".to_string(),
    ///         addr: Ipv4Addr::new(127, 0, 0, 1),
    ///     })
    /// );
    /// ```
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> {
        builders::debug_struct_new(self, name)
    }

    /// Creates a [`DebugTuple`] builder designed to assist with creation of
    /// [`Debug`] implementations for tuple structs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debug2::{pprint, Debug, Formatter};
    /// use std::fmt;
    /// use std::marker::PhantomData;
    ///
    /// struct Foo<T>(i32, String, PhantomData<T>);
    ///
    /// impl<T> Debug for Foo<T> {
    ///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///             .field(&self.0)
    ///             .field(&self.1)
    ///             .field(&format!("_"))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo(10, \"Hello\", \"_\")",
    ///     pprint(Foo(10, "Hello".to_string(), PhantomData::<u8>))
    /// );
    /// ```
    pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b, 'a> {
        builders::debug_tuple_new(self, name)
    }

    /// Creates a [`DebugList`] builder designed to assist with creation of
    /// [`Debug`] implementations for list-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debug2::{pprint, Debug, Formatter};
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    ///         fmt.debug_list().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(pprint(Foo(vec![10, 11])), "[10, 11]");
    /// ```
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> {
        builders::debug_list_new(self)
    }

    /// Creates a [`DebugSet`] builder designed to assist with creation of
    /// [`Debug`] implementations for set-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debug2::{pprint, Debug, Formatter};
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    ///         fmt.debug_set().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(pprint(Foo(vec![10, 11])), "{10, 11}");
    /// ```
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> {
        builders::debug_set_new(self)
    }

    /// Creates a [`DebugMap`] builder designed to assist with creation of
    /// [`Debug`] implementations for map-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use debug2::{pprint, Debug, Formatter};
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    ///         fmt.debug_map()
    ///             .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     r#"{"A": 10, "B": 11}"#
    /// );
    /// ```
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> {
        builders::debug_map_new(self)
    }
}

impl<'a> Formatter<'a> {
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
    where
        'b: 'c,
        F: FnOnce(&'b mut (dyn Write + 'b)) -> &'c mut (dyn Write + 'c),
    {
        Formatter {
            // We want to change this
            buf: wrap(self.buf),

            // And preserve these
            mode: self.mode

            // flags: self.flags,
            // fill: self.fill,
            // align: self.align,
            // width: self.width,
            // precision: self.precision,
        }
    }

    fn is_pretty(&self) -> bool {
        self.mode == Mode::Pretty
    }
}
