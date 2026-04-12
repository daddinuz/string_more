use std::collections::{BTreeMap, HashMap};

use string_more::{EncodeUtf8, StrExt, StringExt};

#[test]
fn encode_utf8() {
    const SEED: [&str; 4] = ["", "·", "x", "Hello world!"];

    for init in SEED {
        let sut = init.to_string();
        assert_eq!(EncodeUtf8::encode_utf8(&sut, &mut ()), init);
    }
}

#[test]
fn next_char_boundary() {
    const SEED: [(&str, usize, usize); 8] = [
        ("", 0, 0),
        ("", 1, 0),
        ("a", 0, 0),
        ("a", 1, 1),
        ("·", 0, 0),
        ("·", 1, 2),
        ("·", 2, 2),
        ("🦀", 2, 4),
    ];

    for (sut, index, expected) in SEED {
        assert_eq!(sut.next_char_boundary(index), expected);
    }

    assert_eq!("abc".next_char_boundary(4), 3);
    assert_eq!("🦀".next_char_boundary(5), 4);
}

#[test]
fn previous_char_boundary() {
    const SEED: [(&str, usize, usize); 8] = [
        ("", 0, 0),
        ("", 1, 0),
        ("a", 0, 0),
        ("a", 1, 1),
        (".", 0, 0),
        ("·", 1, 0),
        ("·", 2, 2),
        ("🦀", 2, 0),
    ];

    for (sut, index, expected) in SEED {
        assert_eq!(sut.previous_char_boundary(index), expected);
    }

    assert_eq!("abc".previous_char_boundary(4), 3);
    assert_eq!("🦀".previous_char_boundary(5), 4);
}

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
    const SEED: [(&str, usize, &str); 21] = [
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
        ("\t·\t", 3, "   ·   "),
        ("\tx\t", 3, "   x   "),
        ("\t·\t", 4, "    ·    "),
        ("\tx\t", 4, "    x    "),
        ("\t·\t", 8, "        ·        "),
        ("\tx\t", 8, "        x        "),
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
fn truncate_to_chars() {
    const SEED: [(&str, usize, &str); 9] = [
        ("", 0, ""),
        ("", 3, ""),
        ("abc", 0, ""),
        ("abc", 2, "ab"),
        ("abc", 3, "abc"),
        ("abc", 4, "abc"),
        ("🦀rust", 1, "🦀"),
        ("🦀rust", 2, "🦀r"),
        ("🦀rust", 5, "🦀rust"),
    ];

    for (init, count, expected) in SEED {
        assert_eq!(init.truncate_to_chars(count), expected);
        assert_eq!(init.to_string().truncate_to_chars(count), expected);
    }
}

#[test]
fn pad_to_width() {
    assert_eq!("42".pad_start_to(5, '0'), "00042");
    assert_eq!("42".pad_end_to(5, '0'), "42000");
    assert_eq!("42".center_to(5, ' '), " 42  ");

    assert_eq!("hello".pad_start_to(3, '.'), "hello");
    assert_eq!("hello".pad_end_to(3, '.'), "hello");
    assert_eq!("hello".center_to(3, '.'), "hello");

    assert_eq!("🦀".pad_start_to(3, ' '), "  🦀");
    assert_eq!("🦀".pad_end_to(3, ' '), "🦀  ");
    assert_eq!("🦀".center_to(4, '.'), ".🦀..");
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
fn truncate_to_chars_in_place() {
    const SEED: [(&str, usize, &str); 9] = [
        ("", 0, ""),
        ("", 3, ""),
        ("abc", 0, ""),
        ("abc", 2, "ab"),
        ("abc", 3, "abc"),
        ("abc", 4, "abc"),
        ("🦀rust", 1, "🦀"),
        ("🦀rust", 2, "🦀r"),
        ("🦀rust", 5, "🦀rust"),
    ];

    for (init, count, expected) in SEED {
        let mut sut = init.to_string();
        sut.truncate_to_chars_in_place(count);
        assert_eq!(sut, expected);
    }
}

#[test]
fn pad_to_width_in_place() {
    let mut start = String::from("42");
    start.pad_start_to_in_place(5, '0');
    assert_eq!(start, "00042");

    let mut end = String::from("42");
    end.pad_end_to_in_place(5, '0');
    assert_eq!(end, "42000");

    let mut center = String::from("42");
    center.center_to_in_place(5, ' ');
    assert_eq!(center, " 42  ");

    let mut unchanged = String::from("hello");
    unchanged.center_to_in_place(3, '.');
    assert_eq!(unchanged, "hello");

    let mut unicode = String::from("🦀");
    unicode.center_to_in_place(4, '.');
    assert_eq!(unicode, ".🦀..");
}

#[test]
fn strip_prefix_in_place() {
    const SEED: [(&str, &str, &str); 8] = [
        ("", "", ""),
        ("", "x", ""),
        ("foobar", "", "foobar"),
        ("foobar", "foo", "bar"),
        ("foobar", "bar", "foobar"),
        ("foobar", "foobar", ""),
        ("🦀rust", "🦀", "rust"),
        ("🦀rust", "rust", "🦀rust"),
    ];

    for (init, prefix, expected) in SEED {
        let mut sut = init.to_string();
        sut.strip_prefix_in_place(prefix);
        assert_eq!(sut, expected);
    }
}

#[test]
fn strip_suffix_in_place() {
    const SEED: [(&str, &str, &str); 8] = [
        ("", "", ""),
        ("", "x", ""),
        ("foobar", "", "foobar"),
        ("foobar", "bar", "foo"),
        ("foobar", "foo", "foobar"),
        ("foobar", "foobar", ""),
        ("rust🦀", "🦀", "rust"),
        ("rust🦀", "rust", "rust🦀"),
    ];

    for (init, suffix, expected) in SEED {
        let mut sut = init.to_string();
        sut.strip_suffix_in_place(suffix);
        assert_eq!(sut, expected);
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
    const SEED: [(&str, usize, &str); 32] = [
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
        ("\t·\t", 3, "   ·   "),
        ("\tx\t", 3, "   x   "),
        ("\t·\t", 4, "    ·    "),
        ("\tx\t", 4, "    x    "),
        ("\t·\t", 8, "        ·        "),
        ("\tx\t", 8, "        x        "),
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
    const SEED: [(&str, usize, usize, &str, &str); 9] = [
        ("", 0, 0, "·", ""),
        ("x", 0, 0, "·", "x"),
        ("x", 0, 1, "", "x"),
        ("x", 0, 1, "·", "·x"),
        ("x", 1, 1, "·", "x·"),
        ("x", 1, 1, "", "x"),
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

#[test]
fn replace_in_place() {
    const SEED: [(&str, &str, &str, &str); 15] = [
        ("", "", "", ""),
        ("", "·", "", ""),
        ("·", "", "", "·"),
        ("x", "·", "", "x"),
        ("·", "x", "", "·"),
        ("x", "x", "", ""),
        ("·", "·", "", ""),
        ("x", "x", "·", "·"),
        ("·", "·", "x", "x"),
        ("Hello world!", "!", ".", "Hello world."),
        ("Hello world!", "!", "·", "Hello world·"),
        ("Hello·world!", "·", " ", "Hello world!"),
        ("Hello world!", " world", "", "Hello!"),
        ("Hello world!", "Hello ", "", "world!"),
        ("abc", "abc", "abcabc", "abcabc"),
    ];

    for (init, from, to, expected) in SEED {
        let mut sut = init.to_string();
        sut.replace_in_place(from, to);
        assert_eq!(sut, expected);
    }
}

#[test]
fn levenshtein_distance() {
    const SEED: [(&str, &str, usize); 18] = [
        ("", "", 0),
        ("", "a", 1),
        ("a", "", 1),
        ("abc", "def", 3),
        ("ring", "bring", 1),
        ("string", "ring", 2),
        ("update", "udpate", 2),
        ("kitten", "sitting", 3),
        ("saturday", "sunday", 3),
        ("execution", "intention", 5),
        ("rosettacode", "raisethysword", 8),
        ("rosettacode", "rosettacode", 0),
        ("Āgain", "āgain", 1),
        ("agĀin", "agāin", 1),
        ("cafexĀ", "cafeyȀ", 2),
        ("Āxcafe", "Ȁycafe", 2),
        ("lorem ipsum dolor", "ipsum", 12),
        ("ipsum", "lorem ipsum dolor", 12),
    ];

    for (sut, other, expected) in SEED {
        assert_eq!(sut.levenshtein_distance(other), expected);
    }
}

#[test]
fn hamming_distance() {
    const SEED: [(&str, &str, Option<usize>); 16] = [
        ("", "", Some(0)),
        ("", "a", None),
        ("a", "", None),
        ("abc", "def", Some(3)),
        ("ring", "bring", None),
        ("string", "ring", None),
        ("update", "udpate", Some(2)),
        ("kitten", "sitting", None),
        ("saturday", "sunday", None),
        ("execution", "intention", Some(5)),
        ("rosettacode", "raisethysword", None),
        ("rosettacode", "rosettacode", Some(0)),
        ("Āgain", "āgain", Some(1)),
        ("agĀin", "agāin", Some(1)),
        ("cafexĀ", "cafeyȀ", Some(2)),
        ("Āxcafe", "Ȁycafe", Some(2)),
    ];

    for (sut, other, expected) in SEED {
        assert!(if sut.chars().count() == other.chars().count() {
            expected.is_some()
        } else {
            expected.is_none()
        });
        assert_eq!(sut.hamming_distance(other), expected);
    }
}

#[test]
fn common_prefix() {
    const SEED: [(&str, &str, &str); 8] = [
        ("", "", ""),
        ("", "abc", ""),
        ("abc", "", ""),
        ("foobar", "foobaz", "fooba"),
        ("foobar", "bar", ""),
        ("🦀rust", "🦀ruby", "🦀ru"),
        ("caffè", "caffo", "caff"),
        ("same", "same", "same"),
    ];

    for (sut, other, expected) in SEED {
        assert_eq!(sut.common_prefix(other), expected);
        assert_eq!(sut.to_string().common_prefix(other), expected);
    }
}

#[test]
fn common_suffix() {
    const SEED: [(&str, &str, &str); 8] = [
        ("", "", ""),
        ("", "abc", ""),
        ("abc", "", ""),
        ("foobar", "quxbar", "bar"),
        ("foobar", "foo", ""),
        ("rust🦀", "dust🦀", "ust🦀"),
        ("caffè", "decaffè", "caffè"),
        ("same", "same", "same"),
    ];

    for (sut, other, expected) in SEED {
        assert_eq!(sut.common_suffix(other), expected);
        assert_eq!(sut.to_string().common_suffix(other), expected);
    }
}

#[test]
fn char_frequencies() {
    const SEED: [(&str, &[(char, usize)]); 3] = [
        ("", &[]),
        ("·x·", &[('x', 1), ('·', 2)]),
        ("hello", &[('h', 1), ('e', 1), ('l', 2), ('o', 1)]),
    ];

    for (sut, expected) in SEED {
        assert_eq!(
            sut.char_frequencies::<BTreeMap<_, _>>(),
            expected.iter().map(|(c, f)| (*c, *f)).collect()
        );

        assert_eq!(
            sut.char_frequencies::<HashMap<_, _>>(),
            expected.iter().map(|(c, f)| (*c, *f)).collect()
        );
    }
}

#[test]
fn byte_frequencies() {
    let ascii = "hello";
    assert_eq!(
        ascii.byte_frequencies::<BTreeMap<_, _>>(),
        [(b'h', 1), (b'e', 1), (b'l', 2), (b'o', 1)]
            .into_iter()
            .collect()
    );
    assert_eq!(
        ascii.byte_frequencies::<HashMap<_, _>>(),
        [(b'h', 1), (b'e', 1), (b'l', 2), (b'o', 1)]
            .into_iter()
            .collect()
    );

    let non_ascii = "·x·";
    assert_eq!(
        non_ascii.byte_frequencies::<BTreeMap<_, _>>(),
        [(0xC2_u8, 2), (0xB7_u8, 2), (b'x', 1)]
            .into_iter()
            .collect()
    );
    assert_eq!(
        non_ascii.byte_frequencies::<HashMap<_, _>>(),
        [(0xC2_u8, 2), (0xB7_u8, 2), (b'x', 1)]
            .into_iter()
            .collect()
    );
}

#[test]
fn longest_common_substring() {
    const SEED: [(&str, &str, &str); 18] = [
        ("", "", ""),
        ("bar", "", ""),
        ("", "bar", ""),
        ("foo", "bar", ""),
        ("hello", "hello", "hello"),
        ("lorem ipsum dolor", "ipsum", "ipsum"),
        ("ipsum", "lorem ipsum dolor", "ipsum"),
        ("spĀm", "spām", "sp"),
        ("bananĀ", "bananā", "banan"),
        ("xĀ", "xā", "x"),
        ("xĀ", "xȀ", "x"),
        ("Āx", "āx", "x"),
        ("Āx", "Ȁx", "x"),
        ("Hello·World!", "·World", "·World"),
        ("Hello·World!", "Hello·", "Hello·"),
        ("0123456789", "012345", "012345"),
        ("0123456789", "456789", "456789"),
        ("0123456789", "345678", "345678"),
    ];

    for (sut, other, expected) in SEED {
        assert_eq!(sut.longest_common_substring(other), expected);
    }
}

#[test]
fn longest_common_substring_prefers_more_chars_over_more_bytes() {
    let sut = "abc🦀";
    let other = "zzabcx🦀y";

    assert_eq!(sut.longest_common_substring(other), "abc");
}

#[test]
fn join_strings() {
    let result = ", ".join(["one", "two", "three"]);
    assert_eq!(result, "one, two, three");
}

#[test]
fn join_integers() {
    let result = "-".join([1, 2, 3, 4]);
    assert_eq!(result, "1-2-3-4");
}

#[test]
fn join_single_item() {
    let result = "/".join(["only"]);
    assert_eq!(result, "only");
}

#[test]
fn join_empty_iterator() {
    let result = ";".join(std::iter::empty::<String>());
    assert_eq!(result, "");
}

#[test]
fn join_non_ascii_strings() {
    let result = "·".join(["Ā", "❤️", "Ȁ"]);
    assert_eq!(result, "Ā·❤️·Ȁ");
}

#[test]
fn fill_edge_cases() {
    assert_eq!("abc".fill_start("", 3), "abc");
    assert_eq!("abc".fill_end("", 3), "abc");
    assert_eq!("abc".center("", 3), "abc");

    let mut sut = String::from("abc");
    sut.fill_start_in_place("", 3);
    assert_eq!(sut, "abc");
    sut.fill_end_in_place("", 3);
    assert_eq!(sut, "abc");
    sut.center_in_place("", 3);
    assert_eq!(sut, "abc");
}

#[test]
fn shift_noop_edge_cases() {
    assert_eq!("abc".shift(1, 0, '-'), "abc");
    assert_eq!("abc".shift(1, 2, ""), "abc");

    let mut sut = String::from("abc");
    sut.shift_in_place(1, 0, '-');
    assert_eq!(sut, "abc");

    sut.shift_in_place(1, 2, "");
    assert_eq!(sut, "abc");
}

#[test]
fn expand_tabs_zero_tabsize_edge_cases() {
    assert_eq!("a\tb".expand_tabs(0), "a\tb");
    assert_eq!("".expand_tabs(0), "");

    let mut sut = String::from("a\tb");
    sut.expand_tabs_in_place(0);
    assert_eq!(sut, "a\tb");
}

#[test]
fn replace_in_place_empty_from_is_noop() {
    let mut sut = String::from("hello");
    sut.replace_in_place("", "x");
    assert_eq!(sut, "hello");
}

#[test]
fn longest_common_substring_edge_cases() {
    assert_eq!("abc".longest_common_substring("xyz"), "");
    assert_eq!("".longest_common_substring("xyz"), "");
    assert_eq!("abc".longest_common_substring(""), "");
}

#[test]
fn join_empty_iterator_edge_case() {
    let values: [String; 0] = [];
    assert_eq!(", ".join(values), "");
}

#[test]
#[should_panic]
fn shift_panics_when_index_is_out_of_bounds() {
    let _ = "abc".shift(4, 1, '-');
}

#[test]
#[should_panic]
fn shift_panics_when_index_is_not_on_char_boundary() {
    let _ = "🦀".shift(1, 1, '-');
}

#[test]
#[should_panic]
fn shift_in_place_panics_when_index_is_out_of_bounds() {
    let mut sut = String::from("abc");
    sut.shift_in_place(4, 1, '-');
}

#[test]
#[should_panic]
fn shift_in_place_panics_when_index_is_not_on_char_boundary() {
    let mut sut = String::from("🦀");
    sut.shift_in_place(1, 1, '-');
}
