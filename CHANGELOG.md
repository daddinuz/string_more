# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-04-12

### Changed
- `previous_char_boundary` now clamps out-of-bounds indices to `self.len()`, matching `next_char_boundary`.
- Split the internal sealing traits by purpose and replaced the `Deref`-based `StrExt` implementation with explicit impls for `str` and `String`.
- Removed the `set` convenience method from `StringExt`.
- Moved the test suite out of `src/lib.rs` into `tests/lib.rs`.
- Standardized and expanded the crate, API, and README documentation.

### Fixed
- `longest_common_substring` now compares substrings by `char` length instead of UTF-8 byte length.
- Corrected the documented space complexity of `longest_common_substring` to match the implementation.

### Added
- Added `byte_frequencies` for counting byte occurrences in a string's UTF-8 representation.
- Added `truncate_to_chars` and `truncate_to_chars_in_place` for char-count-based truncation.
- Added `pad_start_to`, `pad_end_to`, `center_to` and their in-place variants for exact char-width padding with a single fill char.
- Added `strip_prefix_in_place` and `strip_suffix_in_place` as mutable counterparts to the standard prefix/suffix stripping methods.
- Added `common_prefix` and `common_suffix` for borrowed shared prefix/suffix queries.
- Added regression coverage for Unicode behavior in `longest_common_substring`.
- Added edge-case coverage for no-op behavior in `shift`, `shift_in_place`, `expand_tabs`, `expand_tabs_in_place`, and `replace_in_place`.
- Added panic coverage for invalid indices in `shift` and `shift_in_place`.
- Added method-level doctest examples and extra API behavior notes.
