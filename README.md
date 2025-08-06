# string_more

`string_more` is a Rust crate that enhances the `String` and `&str` types from the standard library with powerful extension traits: `StringExt` and `StrExt`.
These traits introduce additional methods to efficiently manipulate strings, focusing on minimizing allocations by allowing in-place modifications for `String` and allocating when necessary for `&str`.

- **`StringExt`**: Provides in-place operations for the `String` type, modifying the string directly without creating a new allocation.
- **`StrExt`**: Offers the same operations for `&str`, but allocates and returns a new `String`, leaving the original `&str` intact.

## Features

- **In-place Operations**: Modify `String` values directly with `StringExt`, minimizing memory allocations.
- **Immutable Operations**: Use `StrExt` to perform operations on `&str` that allocate and return new strings.
- **Flexible and Efficient**: Designed to extend the standard string functionality without sacrificing performance.

## Installation

Add `string_more` to your `Cargo.toml`:

```bash
cargo add string_more
```

or edit your Cargo.toml manually by adding:

```toml
[dependencies]
string_more = "0.3"
```

Then, in your Rust code:

```rust
use string_more::{StringExt, StrExt}; // Import traits

fn main() {
    let mut my_string = String::from("  Hello, Rust!  ");
    my_string.trim_in_place(); // In-place operation on String
    println!("{}", my_string); // "Hello, Rust!"

    let s = "Hello, Rust!";
    let new_s = s.center(' ', 2); // Immutable operation on &str
    println!("{}", new_s); // "  Hello, Rust!  "

    println!("{}", "Hello, Rust!".levenshtein_distance("Hello, World!")); // 5
}
```

## Why string_more?

Rust’s standard library provides robust string handling, but when additional flexibility is needed, `string_more` steps in with efficient, allocation-friendly operations.
With both in-place and immutable operations, `string_more` is ideal for optimizing string manipulation in your Rust projects.

## Methods Overview

### `StringExt` (In-place operations for `String`)

- **`trim_in_place`**: Removes leading and trailing whitespace.
  
  ```rust
  let mut s = String::from("  Hello ");
  s.trim_in_place();
  assert_eq!(s, "Hello");
  ```

- **`trim_start_in_place`**: Removes leading whitespace only.

  ```rust
  let mut s = String::from("  Hello ");
  s.trim_start_in_place();
  assert_eq!(s, "Hello ");
  ```

- **`trim_end_in_place`**: Removes trailing whitespace only.

  ```rust
  let mut s = String::from("  Hello ");
  s.trim_end_in_place();
  assert_eq!(s, "  Hello");
  ```

- **`fill_start_in_place`**: Fills the start of the string with the provided `&str`, repeated a specified number of times.

  ```rust
  let mut s = String::from("Hello");
  s.fill_start_in_place("-", 3);
  assert_eq!(s, "---Hello");
  ```

- **`fill_end_in_place`**: Fills the end of the string with the provided `&str`, repeated a specified number of times.

  ```rust
  let mut s = String::from("Hello");
  s.fill_end_in_place("-", 3);
  assert_eq!(s, "Hello---");
  ```

- **`center_in_place`**: Centers the string by padding both the start and end with the same string.

  ```rust
  let mut s = String::from("Hello");
  s.center_in_place("-", 3);
  assert_eq!(s, "---Hello---");
  ```

- **`enclose_in_place`**: Encloses the string with two separate `&str` values, one at the start and one at the end.

  ```rust
  let mut s = String::from("Hello");
  s.enclose_in_place("[", "]");
  assert_eq!(s, "[Hello]");
  ```

- **`expand_tabs_in_place`**: Replaces tab characters with the specified number of spaces.

  ```rust
  let mut s = String::from("Hello\tRust");
  s.expand_tabs_in_place(4);
  assert_eq!(s, "Hello    Rust");
  ```

- **`shift_in_place`**: Shifts the string at the given index repeating fill pattern n times. 

```rust
let s = "HelloWorld!".to_string();
s.shift_in_place(4, 2, ' ');
assert_eq!(s, "Hello  World!")
```

- **`replace_in_place`**: Replaces a substring with another substring. 

```rust
let s = "HelloWorld!".to_string();
s.replace_in_place("World", " world");
assert_eq!(s, "Hello world!")
```


### `StrExt` (Immutable operations for `&str`)

The `StrExt` trait provides operations that return a new `String` rather than modifying the original `&str`.

- **`fill_start`**: Fills the start of the string with the provided `&str` and returns a new `String`.

  ```rust
  let s = "Hello";
  let filled = s.fill_start("-", 3);
  assert_eq!(filled, "---Hello");
  ```

- **`fill_end`**: Fills the end of the string with the provided `&str` and returns a new `String`.

  ```rust
  let s = "Hello";
  let filled = s.fill_end("-", 3);
  assert_eq!(filled, "Hello---");
  ```

- **`center`**: Centers the string with padding on both sides and returns a new `String`.

  ```rust
  let s = "Hello";
  let centered = s.center("-", 3);
  assert_eq!(centered, "---Hello---");
  ```

- **`enclose`**: Encloses the string with two `&str` values, returning a new `String`.

  ```rust
  let s = "Hello";
  let enclosed = s.enclose("[", "]");
  assert_eq!(enclosed, "[Hello]");
  ```

- **`expand_tabs`**: Replaces tab characters with the specified number of spaces, returning a new `String`.

  ```rust
  let s = "Hello\tRust";
  let expanded = s.expand_tabs(4);
  assert_eq!(expanded, "Hello    Rust");
  ```

- **`shift`**: Shifts the string at the given index repeating fill pattern n times, returning a new `String`. 

```rust
let s = "HelloWorld!";
let shifted = s.shift(4, 2, ' ');
assert_eq!(shifted, "Hello  World!")
```

- **`levenshtein_distance`**: Computes the Levenshtein's distance between two strings.

```rust
let s = "kitten";
assert_eq!(s.levenshtein_distance("sitting"), 3);
```

- **`hamming_distance`**: Computes the Hamming's distance between two strings.

```rust
let s = "update";
assert_eq!(s.hamming_distance("udpate"), 2);
```

- **`char_frequencies`**: Computes the frequencies of chars in the string.

```rust
let s = "Hello";
s.char_frequencies::<BTreeMap<_, _>>(); // H:1 e:1 l:2 o:1
```

- **`longest_common_substring`**: Returns the longest common substring.

```rust
let s = "sparrow";
s.longest_common_substring("crow"); // "row"
```

- **`next_char_boundary`**: Returns the byte index of the next char boundary in string starting from index.

```rust
let s = "🦀";
s.next_char_boundary(2); // 4
```

- **`previous_char_boundary`**: Returns the byte index of the previous char boundary in string starting from index.

```rust
let s = "🦀";
s.previous_char_boundary(2); // 0
```

- **`join`**: Joins an iterator of items into a single `String`.

```rust
let result = ", ".join(["one", "two", "three"]);
assert_eq!(result, "one, two, three");
```

## Safety and Coverage

This crate contains a small portion of unsafe code.
All tests run under [miri](https://github.com/rust-lang/miri) and the tests cover about 90% of the code.
You can generate the coverage report using [tarpaulin](https://github.com/xd009642/tarpaulin).

## Contributions

Contributions are always welcome! If you have ideas for new string operations or performance improvements, feel free to open an issue or submit a pull request.

## License

This crate is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
