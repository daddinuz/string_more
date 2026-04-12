# string_more

[![Crates.io](https://img.shields.io/crates/v/string_more.svg)](https://crates.io/crates/string_more)
[![Docs.rs](https://img.shields.io/docsrs/string_more)](https://docs.rs/string_more)
[![License](https://img.shields.io/crates/l/string_more.svg)](https://github.com/daddinuz/string_more/blob/main/LICENSE)

`string_more` extends `String` and `str` with two utility traits:

- `StringExt` for in-place editing on `String`
- `StrExt` for the corresponding immutable operations on string slices

The crate focuses on small, allocation-aware helpers for padding, editing, boundary handling, and simple text analysis.

## Installation

```bash
cargo add string_more
```

or add it manually:

```toml
[dependencies]
string_more = "0.4"
```

See [CHANGELOG.md](CHANGELOG.md) for release notes and breaking changes.

## Quick Start

```rust
use std::collections::BTreeMap;
use string_more::{StrExt, StringExt};

let centered = "rust".center('-', 2);
assert_eq!(centered, "--rust--");

let mut value = String::from(" rust\t");
value.expand_tabs_in_place(2);
value.trim_in_place();
assert_eq!(value, "rust");

assert_eq!("🦀rust".truncate_to_chars(2), "🦀r");
assert_eq!("foobar".common_prefix("foobaz"), "fooba");

let frequencies = "Hello".byte_frequencies::<BTreeMap<_, _>>();
assert_eq!(frequencies[&b'l'], 2);
```

## API Notes

- `StringExt` mutates `String` in place.
- `StrExt` leaves the original value unchanged. Most transformation methods return a new `String`, while some analysis and slicing helpers return borrowed `&str` results.
- Most text operations are based on Unicode scalar values (`char`), not grapheme clusters.
- Methods that accept indices use byte offsets and require valid char boundaries.
- Most text-like inputs accepted by the API can be passed as `char`, `&str`, or `String` through the sealed `EncodeUtf8` trait.

## Frequency Example

`char_frequencies` counts Unicode scalar values, while `byte_frequencies` counts UTF-8 bytes:

```rust
use std::collections::BTreeMap;
use string_more::StrExt;

let s = "·x·";

let char_frequencies = s.char_frequencies::<BTreeMap<_, _>>();
assert_eq!(char_frequencies[&'·'], 2);
assert_eq!(char_frequencies[&'x'], 1);

let byte_frequencies = s.byte_frequencies::<BTreeMap<_, _>>();
assert_eq!(byte_frequencies[&0xC2], 2);
assert_eq!(byte_frequencies[&0xB7], 2);
assert_eq!(byte_frequencies[&b'x'], 1);
```

## Overview

### In-place editing with `StringExt`

- `trim_start_in_place`, `trim_end_in_place`, `trim_in_place`
- `fill_start_in_place`, `fill_end_in_place`, `center_in_place`
- `pad_start_to_in_place`, `pad_end_to_in_place`, `center_to_in_place`
- `enclose_in_place`
- `expand_tabs_in_place`
- `strip_prefix_in_place`, `strip_suffix_in_place`
- `truncate_to_chars_in_place`
- `shift_in_place`
- `replace_in_place`

```rust
use string_more::StringExt;

let mut s = String::from("  foobar\t");
s.expand_tabs_in_place(2);
s.trim_in_place();
s.strip_prefix_in_place("foo");
s.enclose_in_place("[", "]");

assert_eq!(s, "[bar]");
```

### Immutable helpers with `StrExt`

- `fill_start`, `fill_end`, `center`
- `pad_start_to`, `pad_end_to`, `center_to`
- `enclose`
- `expand_tabs`
- `truncate_to_chars`
- `shift`
- `common_prefix`, `common_suffix`
- `longest_common_substring`
- `levenshtein_distance`, `hamming_distance`
- `char_frequencies`, `byte_frequencies`
- `next_char_boundary`, `previous_char_boundary`
- `join`

```rust
use string_more::StrExt;

assert_eq!("42".pad_start_to(5, '0'), "00042");
assert_eq!("foobar".common_suffix("quxbar"), "bar");
assert_eq!("sparrow".longest_common_substring("crow"), "row");
assert_eq!(", ".join(["one", "two", "three"]), "one, two, three");
```

## Selected Methods

### Padding and enclosure

```rust
use string_more::{StrExt, StringExt};

assert_eq!("Hello".fill_start("-", 3), "---Hello");
assert_eq!("Hello".fill_end("-", 3), "Hello---");
assert_eq!("Hello".center("-", 3), "---Hello---");
assert_eq!("42".center_to(5, ' '), " 42  ");

let mut s = String::from("Hello");
s.enclose_in_place("[", "]");
assert_eq!(s, "[Hello]");
```

### Truncation and prefix/suffix helpers

```rust
use string_more::{StrExt, StringExt};

assert_eq!("🦀rust".truncate_to_chars(2), "🦀r");
assert_eq!("foobar".common_prefix("foobaz"), "fooba");
assert_eq!("foobar".common_suffix("quxbar"), "bar");

let mut s = String::from("foobar");
s.strip_prefix_in_place("foo");
assert_eq!(s, "bar");
```

### Boundary-aware indexing

```rust
use string_more::{StrExt, StringExt};

assert_eq!("🦀".next_char_boundary(2), 4);
assert_eq!("🦀".previous_char_boundary(2), 0);

let mut s = String::from("HelloWorld!");
s.shift_in_place(5, 1, ' ');
assert_eq!(s, "Hello World!");
```

### Distances and substring search

```rust
use string_more::StrExt;

assert_eq!("kitten".levenshtein_distance("sitting"), 3);
assert_eq!("karolin".hamming_distance("kathrin"), Some(3));
assert_eq!("abc🦀".longest_common_substring("zzabcx🦀y"), "abc");
```

## Safety and Coverage

This crate contains a small amount of `unsafe` code in the in-place mutation layer.
The public API is covered by unit tests and doctests.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This crate is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
