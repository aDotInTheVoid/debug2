use std::fmt::{Debug as StdDebug, Error, Result, Write};

mod builders;
mod std_impls;

pub use builders::{DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple};

pub use derive::*;

const MAX_LEN: usize = 80;

pub trait Debug {
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

pub fn pprint_checked<T: Debug>(x: T) -> std::result::Result<String, Error> {
    let flat = flatprint_checked(&x)?;
    if flat.len() <= MAX_LEN {
        Ok(flat)
    } else {
        pprint_mode(x, Mode::Pretty)
    }
}

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
