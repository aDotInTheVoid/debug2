use crate::{Debug, Formatter};
use std::fmt;

struct PadAdapter<'buf, 'state> {
    buf: &'buf mut (dyn fmt::Write + 'buf),
    state: &'state mut PadAdapterState,
}

struct PadAdapterState {
    on_newline: bool,
}

impl Default for PadAdapterState {
    fn default() -> Self {
        PadAdapterState { on_newline: true }
    }
}

impl<'buf, 'state> PadAdapter<'buf, 'state> {
    fn wrap<'slot, 'fmt: 'buf + 'slot>(
        fmt: &'fmt mut Formatter<'_>,
        slot: &'slot mut Option<Self>,
        state: &'state mut PadAdapterState,
    ) -> Formatter<'slot> {
        fmt.wrap_buf(move |buf| {
            *slot = Some(PadAdapter { buf, state });
            slot.as_mut().unwrap()
        })
    }
}

impl fmt::Write for PadAdapter<'_, '_> {
    fn write_str(&mut self, mut s: &str) -> fmt::Result {
        while !s.is_empty() {
            if self.state.on_newline {
                self.buf.write_str("    ")?;
            }

            let split = match s.find('\n') {
                Some(pos) => {
                    self.state.on_newline = true;
                    pos + 1
                }
                None => {
                    self.state.on_newline = false;
                    s.len()
                }
            };
            self.buf.write_str(&s[..split])?;
            s = &s[split..];
        }

        Ok(())
    }
}

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted struct as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_struct`] method.
///
/// # Examples
///
/// ```rust,ignore
/// use std::fmt;
///
/// struct Foo {
///     bar: i32,
///     baz: String,
/// }
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
///         fmt.debug_struct("Foo")
///            .field("bar", &self.bar)
///            .field("baz", &self.baz)
///            .finish()
///     }
/// }
///
/// assert_eq!(
///     format!("{:?}", Foo { bar: 10, baz: "Hello World".to_string() }),
///     "Foo { bar: 10, baz: \"Hello World\" }",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]

pub struct DebugStruct<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
}

pub(super) fn debug_struct_new<'a, 'b>(
    fmt: &'a mut Formatter<'b>,
    name: &str,
) -> DebugStruct<'a, 'b> {
    let result = fmt.write_str(name);
    DebugStruct {
        fmt,
        result,
        has_fields: false,
    }
}

impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
    /// Adds a new field to the generated struct output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     another: String,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar) // We add `bar` field.
    ///            .field("another", &self.another) // We add `another` field.
    ///            // We even add a field which doesn't exist (because why not?).
    ///            .field("not_existing_field", &1)
    ///            .finish() // We're good to go!
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Bar { bar: 10, another: "Hello World".to_string() }),
    ///     "Bar { bar: 10, another: \"Hello World\", not_existing_field: 1 }",
    /// );
    /// ```

    pub fn field(&mut self, name: &str, value: &dyn Debug) -> &mut Self {
        self.result = self.result.and_then(|_| {
            if !self.has_fields {
                self.fmt.write_str(" {\n")?;
            }
            let mut slot = None;
            let mut state = Default::default();
            let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
            writer.write_str(name)?;
            writer.write_str(": ")?;
            value.fmt(&mut writer)?;
            writer.write_str(",\n")
        });

        self.has_fields = true;
        self
    }

    /// Marks the struct as non-exhaustive, indicating to the reader that there are some other
    /// fields that are not shown in the debug representation.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     hidden: f32,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar)
    ///            .finish_non_exhaustive() // Show that some other field(s) exist.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Bar { bar: 10, hidden: 1.0 }),
    ///     "Bar { bar: 10, .. }",
    /// );
    /// ```

    pub fn finish_non_exhaustive(&mut self) -> fmt::Result {
        self.result = self.result.and_then(|_| {
            if self.has_fields {
                let mut slot = None;
                let mut state = Default::default();
                let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
                writer.write_str("..\n")?;
                self.fmt.write_str("}")
            } else {
                self.fmt.write_str(" { .. }")
            }
        });
        self.result
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     baz: String,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar)
    ///            .field("baz", &self.baz)
    ///            .finish() // You need to call it to "finish" the
    ///                      // struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Bar { bar: 10, baz: "Hello World".to_string() }),
    ///     "Bar { bar: 10, baz: \"Hello World\" }",
    /// );
    /// ```

    pub fn finish(&mut self) -> fmt::Result {
        if self.has_fields {
            self.result = self.result.and_then(|_| self.fmt.write_str("}"));
        }
        self.result
    }
}

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted tuple as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_tuple`] method.
///
/// # Examples
///
/// ```rust,ignore
/// use std::fmt;
///
/// struct Foo(i32, String);
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
///         fmt.debug_tuple("Foo")
///            .field(&self.0)
///            .field(&self.1)
///            .finish()
///     }
/// }
///
/// assert_eq!(
///     format!("{:?}", Foo(10, "Hello World".to_string())),
///     "Foo(10, \"Hello World\")",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]

pub struct DebugTuple<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    result: fmt::Result,
    fields: usize,
}

pub(super) fn debug_tuple_new<'a, 'b>(
    fmt: &'a mut Formatter<'b>,
    name: &str,
) -> DebugTuple<'a, 'b> {
    let result = fmt.write_str(name);
    DebugTuple {
        fmt,
        result,
        fields: 0,
    }
}

impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
    /// Adds a new field to the generated tuple struct output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(i32, String);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///            .field(&self.0) // We add the first field.
    ///            .field(&self.1) // We add the second field.
    ///            .finish() // We're good to go!
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(10, "Hello World".to_string())),
    ///     "Foo(10, \"Hello World\")",
    /// );
    /// ```

    pub fn field(&mut self, value: &dyn Debug) -> &mut Self {
        self.result = self.result.and_then(|_| {
            if self.fields == 0 {
                self.fmt.write_str("(\n")?;
            }
            let mut slot = None;
            let mut state = Default::default();
            let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
            value.fmt(&mut writer)?;
            writer.write_str(",\n")
        });

        self.fields += 1;
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(i32, String);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///            .field(&self.0)
    ///            .field(&self.1)
    ///            .finish() // You need to call it to "finish" the
    ///                      // tuple formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(10, "Hello World".to_string())),
    ///     "Foo(10, \"Hello World\")",
    /// );
    /// ```

    pub fn finish(&mut self) -> fmt::Result {
        if self.fields > 0 {
            self.result = self.result.and_then(|_| self.fmt.write_str(")"));
        }
        self.result
    }
}

struct DebugInner<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
}

impl<'a, 'b: 'a> DebugInner<'a, 'b> {
    fn entry(&mut self, entry: &dyn Debug) {
        self.result = self.result.and_then(|_| {
            if !self.has_fields {
                self.fmt.write_str("\n")?;
            }
            let mut slot = None;
            let mut state = Default::default();
            let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
            entry.fmt(&mut writer)?;
            writer.write_str(",\n")
        });

        self.has_fields = true;
    }
}

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted set of items as a part
/// of your [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_set`] method.
///
/// # Examples
///
/// ```rust,ignore
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
/// assert_eq!(
///     format!("{:?}", Foo(vec![10, 11])),
///     "{10, 11}",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]

pub struct DebugSet<'a, 'b: 'a> {
    inner: DebugInner<'a, 'b>,
}

pub(super) fn debug_set_new<'a, 'b>(fmt: &'a mut Formatter<'b>) -> DebugSet<'a, 'b> {
    let result = fmt.write_str("{");
    DebugSet {
        inner: DebugInner {
            fmt,
            result,
            has_fields: false,
        },
    }
}

impl<'a, 'b: 'a> DebugSet<'a, 'b> {
    /// Adds a new entry to the set output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_set()
    ///            .entry(&self.0) // Adds the first "entry".
    ///            .entry(&self.1) // Adds the second "entry".
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11], vec![12, 13])),
    ///     "{[10, 11], [12, 13]}",
    /// );
    /// ```

    pub fn entry(&mut self, entry: &dyn Debug) -> &mut Self {
        self.inner.entry(entry);
        self
    }

    /// Adds the contents of an iterator of entries to the set output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_set()
    ///            .entries(self.0.iter()) // Adds the first "entry".
    ///            .entries(self.1.iter()) // Adds the second "entry".
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11], vec![12, 13])),
    ///     "{10, 11, 12, 13}",
    /// );
    /// ```

    pub fn entries<D, I>(&mut self, entries: I) -> &mut Self
    where
        D: Debug,
        I: IntoIterator<Item = D>,
    {
        for entry in entries {
            self.entry(&entry);
        }
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_set()
    ///            .entries(self.0.iter())
    ///            .finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11])),
    ///     "{10, 11}",
    /// );
    /// ```

    pub fn finish(&mut self) -> fmt::Result {
        self.inner
            .result
            .and_then(|_| self.inner.fmt.write_str("}"))
    }
}

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted list of items as a part
/// of your [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_list`] method.
///
/// # Examples
///
/// ```rust,ignore
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
/// assert_eq!(
///     format!("{:?}", Foo(vec![10, 11])),
///     "[10, 11]",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
pub struct DebugList<'a, 'b: 'a> {
    inner: DebugInner<'a, 'b>,
}

pub(super) fn debug_list_new<'a, 'b>(fmt: &'a mut Formatter<'b>) -> DebugList<'a, 'b> {
    let result = fmt.write_str("[");
    DebugList {
        inner: DebugInner {
            fmt,
            result,
            has_fields: false,
        },
    }
}

impl<'a, 'b: 'a> DebugList<'a, 'b> {
    /// Adds a new entry to the list output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_list()
    ///            .entry(&self.0) // We add the first "entry".
    ///            .entry(&self.1) // We add the second "entry".
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11], vec![12, 13])),
    ///     "[[10, 11], [12, 13]]",
    /// );
    /// ```

    pub fn entry(&mut self, entry: &dyn Debug) -> &mut Self {
        self.inner.entry(entry);
        self
    }

    /// Adds the contents of an iterator of entries to the list output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_list()
    ///            .entries(self.0.iter())
    ///            .entries(self.1.iter())
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11], vec![12, 13])),
    ///     "[10, 11, 12, 13]",
    /// );
    /// ```

    pub fn entries<D, I>(&mut self, entries: I) -> &mut Self
    where
        D: Debug,
        I: IntoIterator<Item = D>,
    {
        for entry in entries {
            self.entry(&entry);
        }
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_list()
    ///            .entries(self.0.iter())
    ///            .finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![10, 11])),
    ///     "[10, 11]",
    /// );
    /// ```

    pub fn finish(&mut self) -> fmt::Result {
        self.inner
            .result
            .and_then(|_| self.inner.fmt.write_str("]"))
    }
}

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted map as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_map`] method.
///
/// # Examples
///
/// ```rust,ignore
/// use std::fmt;
///
/// struct Foo(Vec<(String, i32)>);
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
///     }
/// }
///
/// assert_eq!(
///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
///     "{\"A\": 10, \"B\": 11}",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]

pub struct DebugMap<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    result: fmt::Result,
    has_fields: bool,
    has_key: bool,
    // The state of newlines is tracked between keys and values
    state: PadAdapterState,
}

pub(super) fn debug_map_new<'a, 'b>(fmt: &'a mut Formatter<'b>) -> DebugMap<'a, 'b> {
    let result = fmt.write_str("{");
    DebugMap {
        fmt,
        result,
        has_fields: false,
        has_key: false,
        state: Default::default(),
    }
}

impl<'a, 'b: 'a> DebugMap<'a, 'b> {
    /// Adds a new entry to the map output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map()
    ///            .entry(&"whole", &self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```

    pub fn entry(&mut self, key: &dyn Debug, value: &dyn Debug) -> &mut Self {
        self.key(key).value(value)
    }

    /// Adds the key part of a new entry to the map output.
    ///
    /// This method, together with `value`, is an alternative to `entry` that
    /// can be used when the complete entry isn't known upfront. Prefer the `entry`
    /// method when it's possible to use.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map()
    ///            .key(&"whole").value(&self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```

    pub fn key(&mut self, key: &dyn Debug) -> &mut Self {
        self.result = self.result.and_then(|_| {
            assert!(
                !self.has_key,
                "attempted to begin a new map entry \
                                    without completing the previous one"
            );

            if !self.has_fields {
                self.fmt.write_str("\n")?;
            }
            let mut slot = None;
            self.state = Default::default();
            let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut self.state);
            key.fmt(&mut writer)?;
            writer.write_str(": ")?;

            self.has_key = true;
            Ok(())
        });

        self
    }

    /// Adds the value part of a new entry to the map output.
    ///
    /// This method, together with `key`, is an alternative to `entry` that
    /// can be used when the complete entry isn't known upfront. Prefer the `entry`
    /// method when it's possible to use.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map()
    ///            .key(&"whole").value(&self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```

    pub fn value(&mut self, value: &dyn Debug) -> &mut Self {
        self.result = self.result.and_then(|_| {
            assert!(
                self.has_key,
                "attempted to format a map value before its key"
            );

            let mut slot = None;
            let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut self.state);
            value.fmt(&mut writer)?;
            writer.write_str(",\n")?;

            self.has_key = false;
            Ok(())
        });

        self.has_fields = true;
        self
    }

    /// Adds the contents of an iterator of entries to the map output.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map()
    ///            // We map our vec so each entries' first field will become
    ///            // the "key".
    ///            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"A\": 10, \"B\": 11}",
    /// );
    /// ```

    pub fn entries<K, V, I>(&mut self, entries: I) -> &mut Self
    where
        K: Debug,
        V: Debug,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in entries {
            self.entry(&k, &v);
        }
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map()
    ///            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
    ///            .finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}", Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"A\": 10, \"B\": 11}",
    /// );
    /// ```

    pub fn finish(&mut self) -> fmt::Result {
        self.result.and_then(|_| {
            assert!(
                !self.has_key,
                "attempted to finish a map with a partial entry"
            );

            self.fmt.write_str("}")
        })
    }
}
