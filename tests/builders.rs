use debug2::{pprint, Debug, Formatter};
use std::fmt::Result;

use insta::assert_snapshot;

macro_rules! check {
    ($e:expr) => {
        assert_snapshot!(pprint($e))
    };
}

mod debug_struct {
    use super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo").finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish()
            }
        }

        check!(Bar);
    }

    #[test]
    fn test_only_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish_non_exhaustive()
            }
        }

        check!(Bar);
    }
}

mod debug_tuple {
    use super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_tuple("Foo").finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish()
            }
        }

        check!(Foo);
    }
}

mod debug_map {
    use debug2::pprint_checked;

    use super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Entry;

        impl Debug for Entry {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().entry(&"bar", &true).finish()
            }
        }

        struct KeyValue;

        impl Debug for KeyValue {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().key(&"bar").value(&true).finish()
            }
        }

        assert_eq!(pprint(Entry), pprint(KeyValue));
        check!(Entry);
    }

    #[test]
    fn test_multiple() {
        struct Entry;

        impl Debug for Entry {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct KeyValue;

        impl Debug for KeyValue {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map()
                    .key(&"bar")
                    .value(&true)
                    .key(&10)
                    .value(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        check!(Entry);
        assert_eq!(pprint(&Entry), pprint(&KeyValue));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map()
                    .entry(&"foo", &Foo)
                    .entry(&Foo, &"world")
                    .finish()
            }
        }

        check!(Bar);
    }

    #[test]
    fn test_entry_err() {
        // Ensure errors in a map entry don't trigger panics (#65231)

        struct ErrorFmt;
        use std::fmt::Error;

        impl Debug for ErrorFmt {
            fn fmt(&self, _: &mut Formatter<'_>) -> Result {
                Err(Error)
            }
        }

        struct KeyValue<K, V>(usize, K, V);

        impl<K, V> Debug for KeyValue<K, V>
        where
            K: Debug,
            V: Debug,
        {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                let mut map = fmt.debug_map();

                for _ in 0..self.0 {
                    map.entry(&self.1, &self.2);
                }

                map.finish()
            }
        }

        assert!(pprint_checked(KeyValue(1, ErrorFmt, "bar")).is_err());
        assert!(pprint_checked(KeyValue(1, "foo", ErrorFmt)).is_err());

        assert!(pprint_checked(KeyValue(2, ErrorFmt, "bar")).is_err());
        assert!(pprint_checked(KeyValue(2, "foo", ErrorFmt)).is_err());
    }

    #[test]
    #[should_panic]
    fn test_invalid_key_when_entry_is_incomplete() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().key(&"bar").key(&"invalid").finish()
            }
        }

        pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_finish_incomplete_entry() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().key(&"bar").finish()
            }
        }

        pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_value_before_key() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_map().value(&"invalid").key(&"bar").finish()
            }
        }

        pprint(Foo);
    }
}

mod debug_set {
    use super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_set().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_set().entry(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_set().entry(&Foo).entry(&"world").finish()
            }
        }

        check!(Bar);
    }
}

mod debug_list {
    use super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_list().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_list().entry(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        check!(Foo);
    }
}
