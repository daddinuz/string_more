use std::ops::Deref;

mod sailed {
    pub trait Sailed {}
}

impl sailed::Sailed for char {}
impl sailed::Sailed for &str {}
impl sailed::Sailed for &mut str {}
impl sailed::Sailed for String {}

pub trait EncodeUtf8: sailed::Sailed {
    type Buf: Default;

    fn encode_utf8<'a>(&'a self, buf: &'a mut Self::Buf) -> &'a str;
}

pub trait StrExt: sailed::Sailed {
    fn fill_start(&self, fill: impl EncodeUtf8, times: usize) -> String;

    fn fill_end(&self, fill: impl EncodeUtf8, times: usize) -> String;

    fn center(&self, fill: impl EncodeUtf8, times: usize) -> String;

    fn enclose(&self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8) -> String;

    fn expand_tabs(&self, tabsize: usize) -> String;

    fn shift(&self, index: usize, count: usize, fill: impl EncodeUtf8) -> String;
}

pub trait StringExt: StrExt {
    fn set(&mut self, s: &str);

    fn trim_start_in_place(&mut self);

    fn trim_end_in_place(&mut self);

    fn trim_in_place(&mut self);

    fn fill_start_in_place(&mut self, fill: impl EncodeUtf8, times: usize);

    fn fill_end_in_place(&mut self, fill: impl EncodeUtf8, times: usize);

    fn center_in_place(&mut self, fill: impl EncodeUtf8, times: usize);

    fn enclose_in_place(&mut self, fill_start: impl EncodeUtf8, fill_end: impl EncodeUtf8);

    fn expand_tabs_in_place(&mut self, tabsize: usize);

    fn shift_in_place(&mut self, index: usize, count: usize, fill: impl EncodeUtf8);
}

impl<T> StrExt for T
where
    T: sailed::Sailed + Deref<Target = str>,
{
    fn fill_start(&self, fill: impl EncodeUtf8, times: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * times + self.len());

        for _ in 0..times {
            string.push_str(fill);
        }

        string.push_str(self);
        string
    }

    fn fill_end(&self, fill: impl EncodeUtf8, times: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * times + self.len());

        string.push_str(self);
        for _ in 0..times {
            string.push_str(fill);
        }

        string
    }

    fn center(&self, fill: impl EncodeUtf8, times: usize) -> String {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);
        let mut string = String::with_capacity(fill.len() * 2 * times + self.len());

        for _ in 0..times {
            string.push_str(fill);
        }

        string.push_str(self);

        for _ in 0..times {
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

    fn expand_tabs(&self, tabsize: usize) -> String {
        if tabsize == 0 || self.is_empty() {
            return self.to_string();
        }

        let mut string = String::with_capacity(self.len());
        let mut source = self.deref();
        let expand = match tabsize {
            2 => |s: &mut String, _: usize| s.push_str("  "),
            4 => |s: &mut String, _: usize| s.push_str("    "),
            8 => |s: &mut String, _: usize| s.push_str("        "),
            _ => |s: &mut String, tabsize: usize| {
                s.reserve(tabsize);
                for _ in 0..tabsize {
                    s.push(' ');
                }
            },
        };

        while let Some(i) = source.find('\t') {
            string.push_str(&source[..i]);
            expand(&mut string, tabsize);
            source = &source[i + 1..];
        }

        string.push_str(source);
        string
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
}

impl StringExt for String {
    fn set(&mut self, s: &str) {
        self.clear();
        self.push_str(s);
    }

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

    fn fill_start_in_place(&mut self, fill: impl EncodeUtf8, times: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || times == 0 {
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();
            let additional = fill.len() * times;

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(0..bytes_len, additional);

            for i in (0..fill.len() * times).step_by(fill.len().max(1)) {
                bytes[i..i + fill.len()].copy_from_slice(fill.as_bytes());
            }
        }
    }

    fn fill_end_in_place(&mut self, fill: impl EncodeUtf8, times: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || times == 0 {
            return;
        }

        self.reserve(fill.len() * times);
        for _ in 0..times {
            self.push_str(fill);
        }
    }

    fn center_in_place(&mut self, fill: impl EncodeUtf8, times: usize) {
        let mut buf = Default::default();
        let fill = fill.encode_utf8(&mut buf);

        if fill.is_empty() || times == 0 {
            return;
        }

        #[allow(clippy::uninit_vec)]
        unsafe {
            let bytes = self.as_mut_vec();
            let bytes_len = bytes.len();
            let additional = fill.len() * times * 2;

            bytes.reserve(additional);
            bytes.set_len(bytes_len + additional);
            bytes.copy_within(0..bytes_len, additional / 2);

            for i in (0..times * fill.len()).step_by(fill.len()) {
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

    fn expand_tabs_in_place(&mut self, tabsize: usize) {
        if tabsize == 0 || self.is_empty() {
            return;
        }

        let mut i = 0;
        while i < self.len() {
            if self[i..].starts_with('\t') {
                unsafe { self.as_mut_vec()[i..i + 1].copy_from_slice(" ".as_bytes()) }
                // self.replace_range(i..i + 1, " ");
                self.shift_in_place(i, tabsize.saturating_sub(1), ' ');
                i += tabsize;
            } else {
                i += 1;
                while !self.is_char_boundary(i) {
                    i += 1;
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::{StrExt, StringExt};

    #[test]
    fn fill_start() {
        const SEED: [(&str, &str, usize, &str); 18] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", "-", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("xx", "--", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "x"),
            ("x", "", 1, "x"),
            ("x", "-", 1, "-x"),
            ("xx", "-", 1, "-xx"),
            ("xx", "--", 1, "--xx"),
            ("", "", 2, ""),
            ("", "x", 2, "xx"),
            ("x", "", 2, "x"),
            ("x", "-", 2, "--x"),
            ("xx", "-", 2, "--xx"),
            ("xx", "--", 2, "----xx"),
        ];

        for (init, fill, times, expected) in SEED {
            let sut = init.fill_start(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }

        for (init, fill, times, expected) in SEED {
            let sut = init.to_string().fill_start(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn fill_end() {
        const SEED: [(&str, &str, usize, &str); 18] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", "-", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("xx", "--", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "x"),
            ("x", "", 1, "x"),
            ("x", "-", 1, "x-"),
            ("xx", "-", 1, "xx-"),
            ("xx", "--", 1, "xx--"),
            ("", "", 2, ""),
            ("", "x", 2, "xx"),
            ("x", "", 2, "x"),
            ("x", "-", 2, "x--"),
            ("xx", "-", 2, "xx--"),
            ("xx", "--", 2, "xx----"),
        ];

        for (init, fill, times, expected) in SEED {
            let sut = init.fill_end(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }

        for (init, fill, times, expected) in SEED {
            let sut = init.to_string().fill_end(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn center() {
        const SEED: [(&str, &str, usize, &str); 18] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", " ", 0, "x"),
            ("x", "--", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "xx"),
            ("x", "", 1, "x"),
            ("x", " ", 1, " x "),
            ("x", "--", 1, "--x--"),
            ("xx", "-", 1, "-xx-"),
            ("", "", 2, ""),
            ("", "x", 2, "xxxx"),
            ("x", "", 2, "x"),
            ("x", " ", 2, "  x  "),
            ("x", "--", 2, "----x----"),
            ("xx", "-", 2, "--xx--"),
        ];

        for (init, fill, times, expected) in SEED {
            let sut = init.center(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }

        for (init, fill, times, expected) in SEED {
            let sut = init.to_string().center(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn enclose() {
        const SEED: [(&str, &str, &str, &str); 21] = [
            ("", "", "", ""),
            ("", "(", "", "("),
            ("", "", ")", ")"),
            ("", "((", "", "(("),
            ("", "", "))", "))"),
            ("", "(", ")", "()"),
            ("", "((", "))", "(())"),
            ("x", "", "", "x"),
            ("x", "(", "", "(x"),
            ("x", "", ")", "x)"),
            ("x", "(", ")", "(x)"),
            ("x", "((", "", "((x"),
            ("x", "", "))", "x))"),
            ("x", "((", "))", "((x))"),
            ("xx", "", "", "xx"),
            ("xx", "(", "", "(xx"),
            ("xx", "", ")", "xx)"),
            ("xx", "(", ")", "(xx)"),
            ("xx", "((", "", "((xx"),
            ("xx", "", "))", "xx))"),
            ("xx", "((", "))", "((xx))"),
        ];

        for (init, fill_start, fill_end, expected) in SEED {
            let sut = init.enclose(fill_start, fill_end);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill_start: \"{fill_start}\" fill_end: \"{fill_end}\" expected: \"{expected}\""
            );
        }

        for (init, fill_start, fill_end, expected) in SEED {
            let sut = init.to_string().enclose(fill_start, fill_end);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill_start: \"{fill_start}\" fill_end: \"{fill_end}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn expand_tabs() {
        const SEED: [(&str, usize, &str); 15] = [
            ("", 0, ""),
            ("", 1, ""),
            ("", 2, ""),
            ("\t", 0, "\t"),
            ("\t", 1, " "),
            ("\t", 2, "  "),
            ("\tx\t", 0, "\tx\t"),
            ("\tx\t", 1, " x "),
            ("\tx\t", 2, "  x  "),
            ("x\ty\tx", 0, "x\ty\tx"),
            ("x\ty\tx", 1, "x y x"),
            ("x\ty\tx", 2, "x  y  x"),
            ("\tx\ty\tx\t", 0, "\tx\ty\tx\t"),
            ("\tx\ty\tx\t", 1, " x y x "),
            ("\tx\ty\tx\t", 2, "  x  y  x  "),
        ];

        for (init, tabsize, expected) in SEED {
            let sut = init.expand_tabs(tabsize);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" tabsize: \"{tabsize}\" expected: \"{expected}\""
            );
        }

        for (init, tabsize, expected) in SEED {
            let sut = init.to_string().expand_tabs(tabsize);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" tabsize: \"{tabsize}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn shift() {
        const SEED: [(&str, usize, usize, &str, &str); 7] = [
            ("", 0, 0, "·", ""),
            ("x", 0, 0, "·", "x"),
            ("x", 0, 1, "·", "·x"),
            ("x", 1, 1, "·", "x·"),
            ("xy", 0, 1, "·", "·xy"),
            ("xy", 1, 1, "·", "x·y"),
            ("xy", 2, 1, "·", "xy·"),
        ];

        for (init, index, count, fill, expected) in SEED {
            let sut = init.to_string().shift(index, count, fill);
            assert_eq!(sut, expected);
        }
    }

    #[test]
    fn set() {
        const SEED: [(&str, &str, &str); 3] =
            [("", "", ""), ("", "hello", "hello"), ("hello", "", "")];

        for (init, value, expected) in SEED {
            let mut sut = init.to_string();
            sut.set(value);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" value: \"{value}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn trim_start_in_place() {
        const SEED: [(&str, &str); 8] = [
            ("", ""),
            (" \t\r\n", ""),
            ("x", "x"),
            ("xx", "xx"),
            (" x ", "x "),
            (" \t\r\nHello, world!", "Hello, world!"),
            ("Hello, world!\r\n\t ", "Hello, world!\r\n\t "),
            (" \t\r\nHello, world!\r\n\t ", "Hello, world!\r\n\t "),
        ];

        for (init, expected) in SEED {
            let mut sut = init.to_string();
            sut.trim_start_in_place();
            assert_eq!(sut, expected, "init: \"{init}\" expected: \"{expected}\"");
        }
    }

    #[test]
    fn trim_end_in_place() {
        const SEED: [(&str, &str); 8] = [
            ("", ""),
            (" \t\r\n", ""),
            ("x", "x"),
            ("xx", "xx"),
            (" x ", " x"),
            (" \t\r\nHello, world!", " \t\r\nHello, world!"),
            ("Hello, world!\r\n\t ", "Hello, world!"),
            (" \t\r\nHello, world!\r\n\t ", " \t\r\nHello, world!"),
        ];

        for (init, expected) in SEED {
            let mut sut = init.to_string();
            sut.trim_end_in_place();
            assert_eq!(sut, expected, "init: \"{init}\" expected: \"{expected}\"");
        }
    }

    #[test]
    fn trim_in_place() {
        const SEED: [(&str, &str); 8] = [
            ("", ""),
            (" \t\r\n", ""),
            ("x", "x"),
            ("xx", "xx"),
            (" x ", "x"),
            (" \t\r\nHello, world!", "Hello, world!"),
            ("Hello, world!\r\n\t ", "Hello, world!"),
            (" \t\r\nHello, world!\r\n\t ", "Hello, world!"),
        ];

        for (init, expected) in SEED {
            let mut sut = init.to_string();
            sut.trim_in_place();
            assert_eq!(sut, expected, "init: \"{init}\" expected: \"{expected}\"");
        }
    }

    #[test]
    fn fill_start_in_place() {
        const SEED: [(&str, &str, usize, &str); 19] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", "-", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("xx", "--", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "x"),
            ("x", "", 1, "x"),
            ("x", "-", 1, "-x"),
            ("x", "·", 1, "·x"),
            ("xx", "-", 1, "-xx"),
            ("xx", "--", 1, "--xx"),
            ("", "", 2, ""),
            ("", "x", 2, "xx"),
            ("x", "", 2, "x"),
            ("x", "-", 2, "--x"),
            ("xx", "-", 2, "--xx"),
            ("xx", "--", 2, "----xx"),
        ];

        for (init, fill, times, expected) in SEED {
            let mut sut = init.to_string();
            sut.fill_start_in_place(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn fill_end_in_place() {
        const SEED: [(&str, &str, usize, &str); 19] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", "-", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("xx", "--", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "x"),
            ("x", "", 1, "x"),
            ("x", "-", 1, "x-"),
            ("x", "·", 1, "x·"),
            ("xx", "-", 1, "xx-"),
            ("xx", "--", 1, "xx--"),
            ("", "", 2, ""),
            ("", "x", 2, "xx"),
            ("x", "", 2, "x"),
            ("x", "-", 2, "x--"),
            ("xx", "-", 2, "xx--"),
            ("xx", "--", 2, "xx----"),
        ];

        for (init, fill, times, expected) in SEED {
            let mut sut = init.to_string();
            sut.fill_end_in_place(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn center_in_place() {
        const SEED: [(&str, &str, usize, &str); 19] = [
            ("", "", 0, ""),
            ("", "x", 0, ""),
            ("x", "", 0, "x"),
            ("x", " ", 0, "x"),
            ("x", "--", 0, "x"),
            ("xx", "-", 0, "xx"),
            ("", "", 1, ""),
            ("", "x", 1, "xx"),
            ("x", "", 1, "x"),
            ("x", " ", 1, " x "),
            ("x", "·", 1, "·x·"),
            ("x", "--", 1, "--x--"),
            ("xx", "-", 1, "-xx-"),
            ("", "", 2, ""),
            ("", "x", 2, "xxxx"),
            ("x", "", 2, "x"),
            ("x", " ", 2, "  x  "),
            ("x", "--", 2, "----x----"),
            ("xx", "-", 2, "--xx--"),
        ];

        for (init, fill, times, expected) in SEED {
            let mut sut = init.to_string();
            sut.center_in_place(fill, times);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill: \"{fill}\" times: \"{times}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn enclose_in_place() {
        const SEED: [(&str, &str, &str, &str); 22] = [
            ("", "", "", ""),
            ("", "(", "", "("),
            ("", "", ")", ")"),
            ("", "((", "", "(("),
            ("", "", "))", "))"),
            ("", "(", ")", "()"),
            ("", "((", "))", "(())"),
            ("x", "", "", "x"),
            ("x", "(", "", "(x"),
            ("x", "", ")", "x)"),
            ("x", "(", ")", "(x)"),
            ("x", "((", "", "((x"),
            ("x", "", "))", "x))"),
            ("x", "((", "))", "((x))"),
            ("xx", "", "", "xx"),
            ("xx", "(", "", "(xx"),
            ("xx", "", ")", "xx)"),
            ("xx", "(", ")", "(xx)"),
            ("xx", "((", "", "((xx"),
            ("xx", "", "))", "xx))"),
            ("xx", "((", "))", "((xx))"),
            ("x", "·", "·", "·x·"),
        ];

        for (init, fill_start, fill_end, expected) in SEED {
            let mut sut = init.to_string();
            sut.enclose_in_place(fill_start, fill_end);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" fill_start: \"{fill_start}\" fill_end: \"{fill_end}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn expand_tabs_in_place() {
        const SEED: [(&str, usize, &str); 26] = [
            ("", 0, ""),
            ("", 1, ""),
            ("", 2, ""),
            ("\t", 0, "\t"),
            ("\t", 1, " "),
            ("\t", 2, "  "),
            ("\t·\t", 0, "\t·\t"),
            ("\tx\t", 0, "\tx\t"),
            ("\t·\t", 1, " · "),
            ("\tx\t", 1, " x "),
            ("\t·\t", 2, "  ·  "),
            ("\tx\t", 2, "  x  "),
            ("x\t·\tx", 0, "x\t·\tx"),
            ("x\ty\tx", 0, "x\ty\tx"),
            ("x\t·\tx", 1, "x · x"),
            ("x\ty\tx", 1, "x y x"),
            ("x\t·\tx", 2, "x  ·  x"),
            ("x\ty\tx", 2, "x  y  x"),
            ("\tx\t·\tx\t", 0, "\tx\t·\tx\t"),
            ("\tx\ty\tx\t", 0, "\tx\ty\tx\t"),
            ("\tx\t·\tx\t", 1, " x · x "),
            ("\tx\ty\tx\t", 1, " x y x "),
            ("\tx\ty\tx\t", 2, "  x  y  x  "),
            ("\tx\t·\tx\t", 2, "  x  ·  x  "),
            ("·\tx\t·\tx\t·", 2, "·  x  ·  x  ·"),
            ("\t·\tx\t·\tx\t·\t", 2, "  ·  x  ·  x  ·  "),
        ];

        for (init, tabsize, expected) in SEED {
            let mut sut = init.to_string();
            sut.expand_tabs_in_place(tabsize);
            assert_eq!(
                sut, expected,
                "init: \"{init}\" tabsize: \"{tabsize}\" expected: \"{expected}\""
            );
        }
    }

    #[test]
    fn shift_in_place() {
        const SEED: [(&str, usize, usize, &str, &str); 7] = [
            ("", 0, 0, "·", ""),
            ("x", 0, 0, "·", "x"),
            ("x", 0, 1, "·", "·x"),
            ("x", 1, 1, "·", "x·"),
            ("xy", 0, 1, "·", "·xy"),
            ("xy", 1, 1, "·", "x·y"),
            ("xy", 2, 1, "·", "xy·"),
        ];

        for (init, index, count, fill, expected) in SEED {
            let mut sut = init.to_string();
            sut.shift_in_place(index, count, fill);
            assert_eq!(sut, expected);
        }
    }
}
