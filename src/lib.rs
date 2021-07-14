use std::fmt::{Debug as StdDebug, Error, Result, Write};

mod builders;
mod std_impls;

pub use builders::{DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple};

pub trait Debug2 {
    fn fmt2(&self, f: &mut Formatter2<'_>) -> Result;
}

pub struct Formatter2<'a> {
    buf: &'a mut (dyn Write + 'a),
}

pub fn pprint_checked<T: Debug2>(x: T) -> std::result::Result<String, Error> {
    let mut out = String::new();
    let mut f = Formatter2 { buf: &mut out };
    x.fmt2(&mut f)?;
    Ok(out)
}

pub fn pprint<T: Debug2>(x: T) -> String {
    let mut out = String::new();
    let mut f = Formatter2 { buf: &mut out };
    x.fmt2(&mut f).unwrap();
    out
}

impl<'a> Formatter2<'a> {
    pub fn write_debug<T: StdDebug>(&mut self, val: &T) -> Result {
        write!(self.buf, "{:?}", val)
    }

    pub fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    /// Creates a [`DebugStruct`] builder designed to assist with creation of
    /// [`fmt::Debug`] implementations for structs.
    ///
    /// [`fmt::Debug`]: self::Debug
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::net::Ipv4Addr;
    ///
    /// struct Foo {
    ///     bar: i32,
    ///     baz: String,
    ///     addr: Ipv4Addr,
    /// }
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_struct("Foo")
    ///             .field("bar", &self.bar)
    ///             .field("baz", &self.baz)
    ///             .field("addr", &format_args!("{}", self.addr))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo { bar: 10, baz: \"Hello World\", addr: 127.0.0.1 }",
    ///     format!("{:?}", Foo {
    ///         bar: 10,
    ///         baz: "Hello World".to_string(),
    ///         addr: Ipv4Addr::new(127, 0, 0, 1),
    ///     })
    /// );
    /// ```
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> {
        builders::debug_struct_new(self, name)
    }

    /// Creates a `DebugTuple` builder designed to assist with creation of
    /// `fmt::Debug` implementations for tuple structs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::marker::PhantomData;
    ///
    /// struct Foo<T>(i32, String, PhantomData<T>);
    ///
    /// impl<T> fmt::Debug for Foo<T> {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///             .field(&self.0)
    ///             .field(&self.1)
    ///             .field(&format_args!("_"))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo(10, \"Hello\", _)",
    ///     format!("{:?}", Foo(10, "Hello".to_string(), PhantomData::<u8>))
    /// );
    /// ```
    pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b, 'a> {
        builders::debug_tuple_new(self, name)
    }

    /// Creates a `DebugList` builder designed to assist with creation of
    /// `fmt::Debug` implementations for list-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_list().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "[10, 11]");
    /// ```
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> {
        builders::debug_list_new(self)
    }

    /// Creates a `DebugSet` builder designed to assist with creation of
    /// `fmt::Debug` implementations for set-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_set().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "{10, 11}");
    /// ```
    ///
    /// [`format_args!`]: crate::format_args
    ///
    /// In this more complex example, we use [`format_args!`] and `.debug_set()`
    /// to build a list of match arms:
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Arm<'a, L: 'a, R: 'a>(&'a (L, R));
    /// struct Table<'a, K: 'a, V: 'a>(&'a [(K, V)], V);
    ///
    /// impl<'a, L, R> fmt::Debug for Arm<'a, L, R>
    /// where
    ///     L: 'a + fmt::Debug, R: 'a + fmt::Debug
    /// {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         L::fmt(&(self.0).0, fmt)?;
    ///         fmt.write_str(" => ")?;
    ///         R::fmt(&(self.0).1, fmt)
    ///     }
    /// }
    ///
    /// impl<'a, K, V> fmt::Debug for Table<'a, K, V>
    /// where
    ///     K: 'a + fmt::Debug, V: 'a + fmt::Debug
    /// {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_set()
    ///         .entries(self.0.iter().map(Arm))
    ///         .entry(&Arm(&(format_args!("_"), &self.1)))
    ///         .finish()
    ///     }
    /// }
    /// ```
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> {
        builders::debug_set_new(self)
    }

    /// Creates a `DebugMap` builder designed to assist with creation of
    /// `fmt::Debug` implementations for map-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}",  Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     r#"{"A": 10, "B": 11}"#
    ///  );
    /// ```
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> {
        builders::debug_map_new(self)
    }
}

impl<'a> Formatter2<'a> {
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter2<'c>
    where
        'b: 'c,
        F: FnOnce(&'b mut (dyn Write + 'b)) -> &'c mut (dyn Write + 'c),
    {
        Formatter2 {
            // We want to change this
            buf: wrap(self.buf),
            // And preserve these
            // flags: self.flags,
            // fill: self.fill,
            // align: self.align,
            // width: self.width,
            // precision: self.precision,
        }
    }
}
