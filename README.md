# Debug2

`debug2` is a pritty printing crate based on `std::fmt`

## Why not just use `Debug`

The `Debug` trait is good, but the problem is it is not very good at nested stuctures.
Either you use `{:?}` and get a line that is too long, or a to many lines with not enough
information on them.

```rust
let complex_structure = vec![
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
];
let one_line = format!("{:?}", complex_structure);
assert_eq!(one_line, "[[Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)], [Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)], [Some(1), Some(2), Some(3), None], [Some(2), None], [Some(4), Some(7)]]");
let many_lines = format!("{:#?}", complex_structure);
assert_eq!(many_lines, "[
    [
        Some(
            1,
        ),
        Some(
            2,
        ),
        Some(
            3,
        ),
        None,
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        Some(
            4,
        ),
        Some(
            7,
        ),
    ],
    [
        Some(
            1,
        ),
        Some(
            2,
        ),
        Some(
            3,
        ),
        None,
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        Some(
            4,
        ),
        Some(
            7,
        ),
    ],
    [
        Some(
            1,
        ),
        Some(
            2,
        ),
        Some(
            3,
        ),
        None,
    ],
    [
        Some(
            2,
        ),
        None,
    ],
    [
        Some(
            4,
        ),
        Some(
            7,
        ),
    ],
]")
```

`debug2` aims to be a third alternative, that gets this correct.

```rust
use debug2::pprint;
let complex_structure = vec![
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
    vec![Some(1), Some(2), Some(3), None],
    vec![Some(2), None],
    vec![Some(4), Some(7)],
];
assert_eq!(
    pprint(complex_structure),
    "\
[
    [Some(1), Some(2), Some(3), None],
    [Some(2), None],
    [Some(4), Some(7)],
    [Some(1), Some(2), Some(3), None],
    [Some(2), None],
    [Some(4), Some(7)],
    [Some(1), Some(2), Some(3), None],
    [Some(2), None],
    [Some(4), Some(7)],
]"
);
```

`debug2` provides a `debug2::Debug` trait, which can be derived on your types, and is implemented 
for common types in `std`.

Once your types implement `debug2::Debug`, you can use `debug2::pprint` to convert them to a string.

You can also manually implement `Debug`, using a subset of the API in `std::fmt::Formatter`

## Limitations
- Speed: While doing this will always mean extra work, this crate is paticularly inefficient.
- Prevalence: Almost every type implements `std::fmt::Debug`, but not this type
- The derive isn't great: The deive macro for `std::fmt::Debug` works everywhere. This one
  is kind of basic, and will probably not work everywhere it should.

## Prior art

- [`std::fmt`](https://doc.rust-lang.org/stable/std/fmt/), where much of the code comes from
- [`pprint` from python](https://docs.python.org/3/library/pprint.html)
  , which showed that this sort of thing is doable and great.
- [`ojg`](https://github.com/ohler55/ojg), whose [`pretty`](https://github.com/ohler55/ojg/tree/develop/pretty) module is the basis for this whole thing.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Debug2 by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>