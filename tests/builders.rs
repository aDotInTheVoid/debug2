use debug2::{pprint, Debug2, Formatter2};
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_struct("Foo").finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_tuple("Foo").finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Entry;

        impl Debug2 for Entry {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map().entry(&"bar", &true).finish()
            }
        }

        struct KeyValue;

        impl Debug2 for KeyValue {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map().key(&"bar").value(&true).finish()
            }
        }

        assert_eq!(pprint(Entry), pprint(KeyValue));
        check!(Entry);
    }

    #[test]
    fn test_multiple() {
        struct Entry;

        impl Debug2 for Entry {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct KeyValue;

        impl Debug2 for KeyValue {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for ErrorFmt {
            fn fmt2(&self, _: &mut Formatter2<'_>) -> Result {
                Err(Error)
            }
        }

        struct KeyValue<K, V>(usize, K, V);

        impl<K, V> Debug2 for KeyValue<K, V>
        where
            K: Debug2,
            V: Debug2,
        {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map().key(&"bar").key(&"invalid").finish()
            }
        }

        pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_finish_incomplete_entry() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_map().key(&"bar").finish()
            }
        }

        pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_value_before_key() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_set().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_set().entry(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_list().finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_list().entry(&true).finish()
            }
        }

        check!(Foo);
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
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

        impl Debug2 for Foo {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug2 for Bar {
            fn fmt2(&self, fmt: &mut Formatter2<'_>) -> Result {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        check!(Foo);
    }
}

#[test]
fn test_formatting_parameters_are_forwarded() {
    use std::collections::{BTreeMap, BTreeSet};
    #[derive(Debug)]
    struct Foo {
        bar: u32,
        baz: u32,
    }
    let struct_ = Foo { bar: 1024, baz: 7 };
    let tuple = (1024, 7);
    let list = [1024, 7];
    let mut map = BTreeMap::new();
    map.insert("bar", 1024);
    map.insert("baz", 7);
    let mut set = BTreeSet::new();
    set.insert(1024);
    set.insert(7);

    assert_eq!(format!("{:03?}", struct_), "Foo { bar: 1024, baz: 007 }");
    assert_eq!(format!("{:03?}", tuple), "(1024, 007)");
    assert_eq!(format!("{:03?}", list), "[1024, 007]");
    assert_eq!(format!("{:03?}", map), r#"{"bar": 1024, "baz": 007}"#);
    assert_eq!(format!("{:03?}", set), "{007, 1024}");
    assert_eq!(
        format!("{:#03?}", struct_),
        "
Foo {
    bar: 1024,
    baz: 007,
}
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", tuple),
        "
(
    1024,
    007,
)
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", list),
        "
[
    1024,
    007,
]
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", map),
        r#"
{
    "bar": 1024,
    "baz": 007,
}
    "#
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", set),
        "
{
    007,
    1024,
}
    "
        .trim()
    );
}
