//! Extension traits for `String` and `&str`.
//!
//! `StringExt` provides in-place operations for `String`.\
//! `StrExt` provides the corresponding operations for string slices and returns a new `String`.
//!
//! The crate focuses on small, allocation-aware utilities for formatting, padding, editing, and
//! simple text analysis.
//! Most text operations are based on Unicode scalar values (`char`), not grapheme clusters.
//!
//! # Examples
//!
//! ```rust
//! use string_more::{StrExt, StringExt};
//!
//! let centered = "rust".center('-', 2);
//! assert_eq!(centered, "--rust--");
//!
//! let mut value = String::from(" rust\t");
//! value.expand_tabs_in_place(2);
//! assert_eq!(value, " rust  ");
//! value.trim_in_place();
//! assert_eq!(value, "rust");
//! ```

use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Write};
use std::hash::Hash;

mod sailed {
    pub trait EncodeUtf8 {}

    pub trait StrExt {}

    pub trait StringExt: StrExt {}

    pub trait FrequencyMap<K>: Default {
        fn increment(&mut self, key: K);
    }
}

impl sailed::EncodeUtf8 for char {}
impl sailed::EncodeUtf8 for &str {}
impl sailed::EncodeUtf8 for String {}

impl sailed::StrExt for str {}
impl sailed::StrExt for String {}

impl sailed::StringExt for String {}

impl<K: Ord> sailed::FrequencyMap<K> for BTreeMap<K, usize> {
    fn increment(&mut self, key: K) {
        self.entry(key).and_modify(|n| *n += 1).or_insert(1);
    }
}

impl<K: Eq + Hash> sailed::FrequencyMap<K> for HashMap<K, usize> {
    fn increment(&mut self, key: K) {
        self.entry(key).and_modify(|n| *n += 1).or_insert(1);
    }
}

/// Encodes a text-like value as UTF-8.
///
/// This trait allows methods in `StrExt` and `StringExt` to accept `char`, `&str`, and `String`
/// through a common interface.
///
/// This trait is sealed and cannot be implemented outside this crate.
///
/// # Examples
/// ```rust
/// use string_more::EncodeUtf8;
///
/// let input = 'x';
/// let mut buffer: [u8; 4] = Default::default();
/// let encoded = input.encode_utf8(&mut buffer);
/// assert_eq!(encoded, "x");
/// ```
pub trait EncodeUtf8: sailed::EncodeUtf8 {
    /// Buffer used to hold the encoded value when needed.
    type Buf: Default;

    /// Encodes `self` as UTF-8 and returns it as a string slice.
    fn encode_utf8<'a>(&'a self, buf: &'a mut Self::Buf) -> &'a str;
}

impl EncodeUtf8 for char {
    type Buf = [u8; 4];

    fn encode_utf8<'a>(&'a self, buf: &'a mut Self::Buf) -> &'a str {
        char::encode_utf8(*self, buf)
    }
}

impl EncodeUtf8 for &str {
    type Buf = ();

    fn encode_utf8<'a>(&'a self, _: &'a mut Self::Buf) -> &'a str {
        self
    }
}

impl EncodeUtf8 for String {
    type Buf = ();

    fn encode_utf8<'a>(&'a self, _: &'a mut Self::Buf) -> &'a str {
        self.as_str()
    }
}

/// String utilities for immutable string slices.
///
/// Methods on this trait leave the original value unchanged and return a new `String`.
/// Methods may allocate as needed by the operation being performed.
/// Methods that accept an index use a byte offset unless stated otherwise.
/// Byte indices must lie on char boundaries.
///
/// In this crate, `char` means a Unicode scalar value, not a grapheme cluster.
///
/// # Examples
/// ```rust
/// use string_more::StrExt;
///
/// let example = "example";
/// assert_eq!(example.fill_start("*", 3), "***example");
/// assert_eq!(example.center("-", 4), "----example----");
/// ```
pub trait StrExt: sailed::StrExt {
    /// Returns a new string with `fill` prepended `count` times.
    ///
    /// If `count == 0` or `fill` is empty, returns `self.to_string()`.
    fn fill_start(&self, fill: impl EncodeUtf8, count: usize) -> String;

    /// Returns a new string with `fill` appended `count` times.
    ///
    /// If `count == 0` or `fill` is empty, returns `self.to_string()`.
    fn fill_end(&self, fill: impl EncodeUtf8, count: usize) -> String;

    /// Returns a new string padded on both sides with `fill` repeated `count` times.
    ///
    /// If `count == 0` or `fill` is empty, returns `self.to_string()`.
    fn center(&self, fill: impl EncodeUtf8, count: usize) -> String;

    /// Returns a new string with `fill_start` prepended and `fill_end` appended.
    fn enclose(&self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8) -> String;

    /// Returns a new string where each tab character is replaced with `tab_size` spaces.
    ///
    /// If `tab_size == 0`, returns `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("a\tb".expand_tabs(2), "a  b");
    /// ```
    fn expand_tabs(&self, tab_size: usize) -> String;

    /// Returns a slice truncated to at most `count` chars.
    ///
    /// If `count` is greater than or equal to the string length in chars, returns `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("🦀rust".truncate_to_chars(2), "🦀r");
    /// ```
    fn truncate_to_chars(&self, count: usize) -> &str;

    /// Returns a new string left-padded with `fill` until it reaches `width` chars.
    ///
    /// If the string already has at least `width` chars, returns `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("42".pad_start_to(5, '0'), "00042");
    /// ```
    fn pad_start_to(&self, width: usize, fill: char) -> String;

    /// Returns a new string right-padded with `fill` until it reaches `width` chars.
    ///
    /// If the string already has at least `width` chars, returns `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("42".pad_end_to(5, '0'), "42000");
    /// ```
    fn pad_end_to(&self, width: usize, fill: char) -> String;

    /// Returns a new string padded on both sides with `fill` until it reaches `width` chars.
    ///
    /// If the string already has at least `width` chars, returns `self.to_string()`.
    /// When an odd number of fill chars is needed, the extra char is added at the end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("42".center_to(5, ' '), " 42  ");
    /// ```
    fn center_to(&self, width: usize, fill: char) -> String;

    /// Returns a new string with `count` copies of `fill` inserted at byte index `index`.
    ///
    /// `index` is a byte offset and must lie on a char boundary.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds or does not lie on a char boundary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("ab".shift(1, 2, '-'), "a--b");
    /// ```
    fn shift(&self, index: usize, count: usize, fill: impl EncodeUtf8) -> String;

    /// Returns the Levenshtein distance between `self` and `other`.
    ///
    /// The comparison is performed on `char`s.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n * m)` time and `O(min(n, m))` space.
    fn levenshtein_distance(&self, other: &str) -> usize;

    /// Returns the Hamming distance between `self` and `other`.
    ///
    /// If the strings have different lengths in chars, returns `None`.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n)` time and `O(1)` space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("karolin".hamming_distance("kathrin"), Some(3));
    /// assert_eq!("rust".hamming_distance("rusty"), None);
    /// ```
    fn hamming_distance(&self, other: &str) -> Option<usize>;

    /// Returns the common prefix shared by `self` and `other`.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n)` time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("foobar".common_prefix("foobaz"), "fooba");
    /// ```
    fn common_prefix(&self, other: &str) -> &str;

    /// Returns the common suffix shared by `self` and `other`.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n)` time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("foobar".common_suffix("quxbar"), "bar");
    /// ```
    fn common_suffix(&self, other: &str) -> &str;

    /// Returns the frequency of each char in the string.
    ///
    /// The output type is chosen by the caller and is typically a `BTreeMap<char, usize>` or
    /// `HashMap<char, usize>`.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n)` time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use string_more::StrExt;
    ///
    /// let frequencies = "hello".char_frequencies::<BTreeMap<_, _>>();
    /// assert_eq!(frequencies[&'l'], 2);
    /// ```
    fn char_frequencies<M: sailed::FrequencyMap<char>>(&self) -> M;

    /// Returns the frequency of each byte in the string.
    ///
    /// The output type is chosen by the caller and is typically a `BTreeMap<u8, usize>` or
    /// `HashMap<u8, usize>`.
    ///
    /// Bytes are counted from the UTF-8 representation of the string.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n)` time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use string_more::StrExt;
    ///
    /// let frequencies = "hello".byte_frequencies::<BTreeMap<_, _>>();
    /// assert_eq!(frequencies[&b'l'], 2);
    /// ```
    fn byte_frequencies<M: sailed::FrequencyMap<u8>>(&self) -> M;

    /// Returns the longest common substring between `self` and `other`.
    ///
    /// The comparison is performed on `char`s rather than UTF-8 bytes.
    /// The returned substring always borrows from `self`.
    /// If multiple substrings have the same maximum length, the first match found in `self` is returned.
    /// If no common substring exists, returns `""`.
    ///
    /// # Complexity
    ///
    /// Runs in `O(n * m)` time and `O(n + m)` space, where `n` is the number of chars in `self`
    /// and `m` is the number of chars in `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("abc🦀".longest_common_substring("zzabcx🦀y"), "abc");
    /// ```
    fn longest_common_substring(&self, other: &str) -> &str;

    /// Returns the next char boundary at or after `index`.
    ///
    /// If `index` is already on a char boundary, it is returned unchanged.
    /// Both `0` and `self.len()` are considered valid char boundaries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("🦀".next_char_boundary(1), 4);
    /// assert_eq!("éa".next_char_boundary(2), 2);
    /// ```
    fn next_char_boundary(&self, index: usize) -> usize;

    /// Returns the previous char boundary at or before `index`.
    ///
    /// If `index` is already on a char boundary, it is returned unchanged.
    /// Both `0` and `self.len()` are considered valid char boundaries.
    /// If `index > self.len()`, returns `self.len()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StrExt;
    ///
    /// assert_eq!("🦀".previous_char_boundary(1), 0);
    /// assert_eq!("éa".previous_char_boundary(2), 2);
    /// ```
    fn previous_char_boundary(&self, index: usize) -> usize;

    /// Joins the items from `iterable` into a single string, using `self` as the separator.
    fn join<T: Display>(&self, iterable: impl IntoIterator<Item = T>) -> String;
}

impl StrExt for str {
    fn fill_start(&self, fill: impl EncodeUtf8, count: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * count + self.len());

        for _ in 0..count {
            string.push_str(fill);
        }

        string.push_str(self);
        string
    }

    fn fill_end(&self, fill: impl EncodeUtf8, count: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * count + self.len());

        string.push_str(self);
        for _ in 0..count {
            string.push_str(fill);
        }

        string
    }

    fn center(&self, fill: impl EncodeUtf8, count: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * 2 * count + self.len());

        for _ in 0..count {
            string.push_str(fill);
        }

        string.push_str(self);

        for _ in 0..count {
            string.push_str(fill);
        }

        string
    }

    fn enclose(&self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8) -> String {
        let (mut start_buf, mut end_buf) = (Default::default(), Default::default());
        let (start, end) = (
            fill_start.encode_utf8(&mut start_buf),
            fill_end.encode_utf8(&mut end_buf),
        );
        let mut string = String::with_capacity(start.len() + self.len() + end.len());
        string.push_str(start);
        string.push_str(self);
        string.push_str(end);
        string
    }

    fn expand_tabs(&self, tab_size: usize) -> String {
        if tab_size == 0 || self.is_empty() {
            return self.to_string();
        }

        let mut string = String::with_capacity(self.len());
        let mut source = self;
        let expand = match tab_size {
            2 => |s: &mut String, _: usize| s.push_str("  "),
            4 => |s: &mut String, _: usize| s.push_str("    "),
            8 => |s: &mut String, _: usize| s.push_str("        "),
            _ => |s: &mut String, tab_size: usize| {
                s.reserve(tab_size);
                for _ in 0..tab_size {
                    s.push(' ');
                }
            },
        };

        while let Some(i) = source.find('\t') {
            string.push_str(&source[..i]);
            expand(&mut string, tab_size);
            source = &source[i + 1..];
        }

        string.push_str(source);
        string
    }

    fn truncate_to_chars(&self, count: usize) -> &str {
        let end = self
            .char_indices()
            .map(|(index, _)| index)
            .nth(count)
            .unwrap_or(self.len());
        &self[..end]
    }

    fn pad_start_to(&self, width: usize, fill: char) -> String {
        let missing = width.saturating_sub(self.chars().count());
        self.fill_start(fill, missing)
    }

    fn pad_end_to(&self, width: usize, fill: char) -> String {
        let missing = width.saturating_sub(self.chars().count());
        self.fill_end(fill, missing)
    }

    fn center_to(&self, width: usize, fill: char) -> String {
        let missing = width.saturating_sub(self.chars().count());
        if missing == 0 {
            return self.to_string();
        }

        let left = missing / 2;
        let right = missing - left;
        self.fill_start(fill, left).fill_end(fill, right)
    }

    fn shift(&self, index: usize, count: usize, fill: impl EncodeUtf8) -> String {
        assert!(self.is_char_boundary(index));
        assert!(index <= self.len());

        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if count == 0 || fill.is_empty() {
            return self.to_string();
        }

        let mut s = self[..index].to_string();
        for _ in 0..count {
            s.push_str(fill);
        }

        s.push_str(&self[index..]);
        s
    }

    // Adapted and slightly modified from the code found on
    // rosettacode: https://rosettacode.org/wiki/Levenshtein_distance#C++
    // by Martin Ettl; I don't know who you are but thank you! <3
    fn levenshtein_distance(&self, other: &str) -> usize {
        // --- [this is not part of the adapted code from C++] ---
        let (source, target) = (self, other);

        // optimize cases where source and target are the same instance.
        if source.as_ptr() == target.as_ptr() && source.len() == target.len() {
            return 0;
        }

        // optimize memory allocations by stripping common
        // suffix and prefix between source and target.

        let mut end = source
            .bytes()
            .rev()
            .zip(target.bytes().rev())
            .take_while(|(l, r)| l == r)
            .count();

        // ensure end happens on a valid char boundary
        while !source.is_char_boundary(source.len() - end) {
            end -= 1;
        }

        // strip common suffix
        let (source, target) = (&source[..source.len() - end], &target[..target.len() - end]);

        let mut start = source
            .bytes()
            .zip(target.bytes())
            .take_while(|(l, r)| l == r)
            .count();

        // ensure start happens on a valid char boundary
        while !source.is_char_boundary(start) {
            start -= 1;
        }

        // strip common prefix
        let (source, target) = (&source[start..], &target[start..]);

        // -- [the adapted code from C++ starts here] ---

        if source.is_empty() {
            return target.chars().count();
        }

        if target.is_empty() {
            return source.chars().count();
        }

        // --- [this is not part of the adapted code from C++] ---
        // micro optimization: `costs` vector has the same cardinality
        // of target's chars so we try to reduce memory allocations by
        // assigning the smallest string (bytes len) to target.
        // the memory reduction actually depends on the chars composing
        // the string, so we optimistically bet that the fewer bytes that
        // make up the string, the fewer chars will be present in that string.
        // this is not always true and in those cases we could even end up
        // allocating more, but those cases should be rare enough to
        // justify this optimization.
        let (source, target) = if source.len() < target.len() {
            (target, source)
        } else {
            (source, target)
        };
        // --------------------------------------------------------------------

        let target_len = target.chars().count();
        let mut costs = (0..=target_len).collect::<Vec<_>>();

        for (source_index, source_char) in source.chars().enumerate() {
            let mut corner = source_index;
            costs[0] = source_index + 1;

            for (target_index, target_char) in target.chars().enumerate() {
                let upper = costs[target_index + 1];

                costs[target_index + 1] = if source_char == target_char {
                    corner
                } else {
                    1 + usize::min(usize::min(costs[target_index], upper), corner)
                };

                corner = upper;
            }
        }

        costs[target_len]
    }

    fn hamming_distance(&self, other: &str) -> Option<usize> {
        let (mut source, mut target) = (self.chars(), other.chars());
        let mut distance = 0;

        loop {
            let source_char = match source.next() {
                Some(c) => c,
                None => {
                    return match target.next() {
                        Some(_) => None,
                        None => Some(distance),
                    }
                }
            };

            let target_char = target.next()?;
            distance += (source_char != target_char) as usize;
        }
    }

    fn common_prefix(&self, other: &str) -> &str {
        let end = self
            .char_indices()
            .zip(other.chars())
            .take_while(|((_, left), right)| left == right)
            .last()
            .map(|((index, left), _)| index + left.len_utf8())
            .unwrap_or(0);

        &self[..end]
    }

    fn common_suffix(&self, other: &str) -> &str {
        let suffix_len = self
            .chars()
            .rev()
            .zip(other.chars().rev())
            .take_while(|(left, right)| left == right)
            .fold(0, |acc, (c, _)| acc + c.len_utf8());

        &self[self.len() - suffix_len..]
    }

    fn char_frequencies<M: sailed::FrequencyMap<char>>(&self) -> M {
        let mut map = M::default();
        self.chars().for_each(|c| map.increment(c));
        map
    }

    fn byte_frequencies<M: sailed::FrequencyMap<u8>>(&self) -> M {
        let mut map = M::default();
        self.bytes().for_each(|c| map.increment(c));
        map
    }

    fn longest_common_substring(&self, other: &str) -> &str {
        if self.is_empty() || other.is_empty() {
            return "";
        }

        let source = self.char_indices().collect::<Vec<_>>();
        let target = other.chars().collect::<Vec<_>>();

        let mut previous_row = vec![0; target.len() + 1];
        let mut current_row = vec![0; target.len() + 1];
        let mut best_len = 0;
        let mut best_end = 0;

        for (source_index, (_, source_char)) in source.iter().enumerate() {
            for (target_index, target_char) in target.iter().enumerate() {
                current_row[target_index + 1] = if source_char == target_char {
                    previous_row[target_index] + 1
                } else {
                    0
                };

                if current_row[target_index + 1] > best_len {
                    best_len = current_row[target_index + 1];
                    best_end = source_index + 1;
                }
            }

            previous_row.fill(0);
            std::mem::swap(&mut previous_row, &mut current_row);
        }

        if best_len == 0 {
            return "";
        }

        let start = source[best_end - best_len].0;
        let end = source
            .get(best_end)
            .map(|(index, _)| *index)
            .unwrap_or(self.len());

        &self[start..end]
    }

    fn next_char_boundary(&self, mut index: usize) -> usize {
        if index > self.len() {
            return self.len();
        }

        while !self.is_char_boundary(index) {
            index += 1;
        }

        index
    }

    fn previous_char_boundary(&self, mut index: usize) -> usize {
        if index > self.len() {
            return self.len();
        }

        while !self.is_char_boundary(index) {
            index -= 1;
        }

        index
    }

    fn join<T: Display>(&self, iterable: impl IntoIterator<Item = T>) -> String {
        let mut iter = iterable.into_iter();
        let mut acc = String::new();

        if let Some(value) = iter.next() {
            write!(acc, "{value}").unwrap();
            iter.try_for_each(|item| write!(acc, "{self}{item}"))
                .unwrap();
        }

        acc
    }
}

impl StrExt for String {
    fn fill_start(&self, fill: impl EncodeUtf8, count: usize) -> String {
        self.as_str().fill_start(fill, count)
    }

    fn fill_end(&self, fill: impl EncodeUtf8, count: usize) -> String {
        self.as_str().fill_end(fill, count)
    }

    fn center(&self, fill: impl EncodeUtf8, count: usize) -> String {
        self.as_str().center(fill, count)
    }

    fn enclose(&self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8) -> String {
        self.as_str().enclose(fill_start, fill_end)
    }

    fn expand_tabs(&self, tab_size: usize) -> String {
        self.as_str().expand_tabs(tab_size)
    }

    fn truncate_to_chars(&self, count: usize) -> &str {
        self.as_str().truncate_to_chars(count)
    }

    fn pad_start_to(&self, width: usize, fill: char) -> String {
        self.as_str().pad_start_to(width, fill)
    }

    fn pad_end_to(&self, width: usize, fill: char) -> String {
        self.as_str().pad_end_to(width, fill)
    }

    fn center_to(&self, width: usize, fill: char) -> String {
        self.as_str().center_to(width, fill)
    }

    fn shift(&self, index: usize, count: usize, fill: impl EncodeUtf8) -> String {
        self.as_str().shift(index, count, fill)
    }

    fn levenshtein_distance(&self, other: &str) -> usize {
        self.as_str().levenshtein_distance(other)
    }

    fn hamming_distance(&self, other: &str) -> Option<usize> {
        self.as_str().hamming_distance(other)
    }

    fn common_prefix(&self, other: &str) -> &str {
        self.as_str().common_prefix(other)
    }

    fn common_suffix(&self, other: &str) -> &str {
        self.as_str().common_suffix(other)
    }

    fn char_frequencies<M: sailed::FrequencyMap<char>>(&self) -> M {
        self.as_str().char_frequencies()
    }

    fn byte_frequencies<M: sailed::FrequencyMap<u8>>(&self) -> M {
        self.as_str().byte_frequencies()
    }

    fn longest_common_substring(&self, other: &str) -> &str {
        self.as_str().longest_common_substring(other)
    }

    fn next_char_boundary(&self, index: usize) -> usize {
        self.as_str().next_char_boundary(index)
    }

    fn previous_char_boundary(&self, index: usize) -> usize {
        self.as_str().previous_char_boundary(index)
    }

    fn join<T: Display>(&self, iterable: impl IntoIterator<Item = T>) -> String {
        self.as_str().join(iterable)
    }
}

/// In-place string utilities for `String`.
///
/// Methods on this trait mutate the string directly instead of returning a new value.
/// Methods may reserve or reallocate as needed by the operation being performed.
/// Methods that accept an index use a byte offset unless stated otherwise.
/// Byte indices must lie on char boundaries.
///
/// # Examples
/// ```rust
/// use string_more::StringExt;
///
/// let mut example = String::from(" example ");
/// example.trim_in_place();
/// assert_eq!(example, "example");
/// example.fill_end_in_place("-", 3);
/// assert_eq!(example, "example---");
/// ```
pub trait StringExt: StrExt + sailed::StringExt {
    /// Removes leading whitespace in place.
    fn trim_start_in_place(&mut self);

    /// Removes trailing whitespace in place.
    fn trim_end_in_place(&mut self);

    /// Removes leading and trailing whitespace in place.
    fn trim_in_place(&mut self);

    /// Prepends `fill` `count` times in place.
    ///
    /// If `count == 0` or `fill` is empty, does nothing.
    fn fill_start_in_place(&mut self, fill: impl EncodeUtf8, count: usize);

    /// Appends `fill` `count` times in place.
    ///
    /// If `count == 0` or `fill` is empty, does nothing.
    fn fill_end_in_place(&mut self, fill: impl EncodeUtf8, count: usize);

    /// Pads both sides with `fill` repeated `count` times in place.
    ///
    /// If `count == 0` or `fill` is empty, does nothing.
    fn center_in_place(&mut self, fill: impl EncodeUtf8, count: usize);

    /// Prepends `fill_start` and appends `fill_end` in place.
    fn enclose_in_place(&mut self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8);

    /// Replaces each tab character with `tab_size` spaces in place.
    ///
    /// If `tab_size == 0`, does nothing.
    fn expand_tabs_in_place(&mut self, tab_size: usize);

    /// Removes `prefix` from the start of the string in place if present.
    ///
    /// If the string does not start with `prefix`, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("foobar");
    /// value.strip_prefix_in_place("foo");
    /// assert_eq!(value, "bar");
    /// ```
    fn strip_prefix_in_place(&mut self, prefix: &str);

    /// Removes `suffix` from the end of the string in place if present.
    ///
    /// If the string does not end with `suffix`, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("foobar");
    /// value.strip_suffix_in_place("bar");
    /// assert_eq!(value, "foo");
    /// ```
    fn strip_suffix_in_place(&mut self, suffix: &str);

    /// Truncates the string to at most `count` chars in place.
    ///
    /// If `count` is greater than or equal to the string length in chars, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("🦀rust");
    /// value.truncate_to_chars_in_place(2);
    /// assert_eq!(value, "🦀r");
    /// ```
    fn truncate_to_chars_in_place(&mut self, count: usize);

    /// Left-pads the string with `fill` until it reaches `width` chars in place.
    ///
    /// If the string already has at least `width` chars, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("42");
    /// value.pad_start_to_in_place(5, '0');
    /// assert_eq!(value, "00042");
    /// ```
    fn pad_start_to_in_place(&mut self, width: usize, fill: char);

    /// Right-pads the string with `fill` until it reaches `width` chars in place.
    ///
    /// If the string already has at least `width` chars, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("42");
    /// value.pad_end_to_in_place(5, '0');
    /// assert_eq!(value, "42000");
    /// ```
    fn pad_end_to_in_place(&mut self, width: usize, fill: char);

    /// Pads the string on both sides with `fill` until it reaches `width` chars in place.
    ///
    /// If the string already has at least `width` chars, does nothing.
    /// When an odd number of fill chars is needed, the extra char is added at the end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("42");
    /// value.center_to_in_place(5, ' ');
    /// assert_eq!(value, " 42  ");
    /// ```
    fn center_to_in_place(&mut self, width: usize, fill: char);

    /// Inserts `count` copies of `fill` at byte index `index` in place.
    ///
    /// `index` is a byte offset and must lie on a char boundary.
    /// If `count == 0` or `fill` is empty, does nothing.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds or does not lie on a char boundary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("ab");
    /// value.shift_in_place(1, 2, '-');
    /// assert_eq!(value, "a--b");
    /// ```
    fn shift_in_place(&mut self, index: usize, count: usize, fill: impl EncodeUtf8);

    /// Replaces all occurrences of `from` with `to` in place.
    ///
    /// If `from` is empty, does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use string_more::StringExt;
    ///
    /// let mut value = String::from("hello world");
    /// value.replace_in_place("world", "rust");
    /// assert_eq!(value, "hello rust");
    /// ```
    fn replace_in_place(&mut self, from: impl EncodeUtf8, to: impl EncodeUtf8);
}

impl StringExt for String {
    fn trim_start_in_place(&mut self) {
        let trimmed = self.trim_start();
        let start = unsafe { trimmed.as_ptr().offset_from(self.as_ptr()) as usize };
        let len = trimmed.len();
        unsafe { self.as_mut_vec().copy_within(start..start + len, 0) };
        self.truncate(len);
    }

    fn trim_end_in_place(&mut self) {
        self.truncate(self.trim_end().len());
    }

    fn trim_in_place(&mut self) {
        let trimmed = self.trim();
        let start = unsafe { trimmed.as_ptr().offset_from(self.as_ptr()) } as usize;
        let len = trimmed.len();
        unsafe { self.as_mut_vec().copy_within(start..start + len, 0) };
        self.truncate(len);
    }

    fn fill_start_in_place(&mut self, fill: impl EncodeUtf8, count: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || count == 0 {
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();
            let additional = fill.len() * count;

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(0..bytes_len, additional);

            for i in (0..fill.len() * count).step_by(fill.len().max(1)) {
                bytes[i..i + fill.len()].copy_from_slice(fill.as_bytes());
            }
        }
    }

    fn fill_end_in_place(&mut self, fill: impl EncodeUtf8, count: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || count == 0 {
            return;
        }

        self.reserve(fill.len() * count);
        for _ in 0..count {
            self.push_str(fill);
        }
    }

    fn center_in_place(&mut self, fill: impl EncodeUtf8, count: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || count == 0 {
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();
            let additional = fill.len() * count * 2;

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(0..bytes_len, additional / 2);

            for i in (0..count * fill.len()).step_by(fill.len()) {
                bytes[i..i + fill.len()].copy_from_slice(fill.as_bytes());
            }

            for i in (bytes_len + additional / 2..bytes_len + additional).step_by(fill.len().max(1))
            {
                bytes[i..i + fill.len()].copy_from_slice(fill.as_bytes());
            }
        }
    }

    fn enclose_in_place(&mut self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8) {
        let mut buf_start = Default::default();
        let fill_start = fill_start.encode_utf8(&mut buf_start);

        let mut buf_end = Default::default();
        let fill_end = fill_end.encode_utf8(&mut buf_end);

        if fill_start.is_empty() && fill_end.is_empty() {
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();
            let additional = fill_start.len() + fill_end.len();

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(0..bytes_len, fill_start.len());
            bytes[..fill_start.len()].copy_from_slice(fill_start.as_bytes());
            bytes[fill_start.len() + bytes_len..].copy_from_slice(fill_end.as_bytes());
        }
    }

    fn expand_tabs_in_place(&mut self, tab_size: usize) {
        if tab_size == 0 || self.is_empty() {
            return;
        }

        let mut i = 0;
        while i < self.len() {
            if self[i..].starts_with('\t') {
                unsafe { self.as_mut_vec()[i..i + 1].copy_from_slice(" ".as_bytes()) }
                // self.replace_range(i..i + 1, " ");
                self.shift_in_place(i, tab_size.saturating_sub(1), ' ');
                i += tab_size;
            } else {
                i = self.next_char_boundary(i + 1);
            }
        }
    }

    fn strip_prefix_in_place(&mut self, prefix: &str) {
        if let Some(stripped) = self.strip_prefix(prefix) {
            let start = self.len() - stripped.len();
            let len = stripped.len();
            unsafe { self.as_mut_vec().copy_within(start..start + len, 0) };
            self.truncate(len);
        }
    }

    fn strip_suffix_in_place(&mut self, suffix: &str) {
        if let Some(stripped) = self.strip_suffix(suffix) {
            self.truncate(stripped.len());
        }
    }

    fn truncate_to_chars_in_place(&mut self, count: usize) {
        let end = self
            .char_indices()
            .map(|(index, _)| index)
            .nth(count)
            .unwrap_or(self.len());
        self.truncate(end);
    }

    fn pad_start_to_in_place(&mut self, width: usize, fill: char) {
        let missing = width.saturating_sub(self.chars().count());
        self.fill_start_in_place(fill, missing);
    }

    fn pad_end_to_in_place(&mut self, width: usize, fill: char) {
        let missing = width.saturating_sub(self.chars().count());
        self.fill_end_in_place(fill, missing);
    }

    fn center_to_in_place(&mut self, width: usize, fill: char) {
        let missing = width.saturating_sub(self.chars().count());
        if missing == 0 {
            return;
        }

        let left = missing / 2;
        let right = missing - left;
        self.fill_start_in_place(fill, left);
        self.fill_end_in_place(fill, right);
    }

    fn shift_in_place(&mut self, index: usize, count: usize, fill: impl EncodeUtf8) {
        assert!(self.is_char_boundary(index));
        assert!(index <= self.len());

        let mut buf = Default::default();
        let fill = EncodeUtf8::encode_utf8(&fill, &mut buf);
        let additional = count * fill.len();

        if count == 0 || fill.is_empty() {
            return;
        }

        if count == 1 {
            self.insert_str(index, fill);
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(index..bytes_len, index + count * fill.len());

            for i in (index..index + count * fill.len()).step_by(fill.len()) {
                bytes[i..i + fill.len()].copy_from_slice(fill.as_bytes())
            }
        }
    }

    fn replace_in_place(&mut self, from: impl EncodeUtf8, to: impl EncodeUtf8) {
        let (mut from_buf, mut to_buf) = (Default::default(), Default::default());
        let from = from.encode_utf8(&mut from_buf);
        let to = to.encode_utf8(&mut to_buf);

        if self.is_empty() || from.is_empty() {
            return;
        }

        let mut offset = 0;
        while let Some(i) = self[offset..].find(from) {
            offset += i;
            self.replace_range(offset..offset + from.len(), to);
            offset += to.len();
        }
    }
}
