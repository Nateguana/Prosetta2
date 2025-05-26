#![cfg(test)]
use std::mem;

use crate::parser::alias_finder::ParsedAliasData;

use super::*;
use itertools::{self, Itertools};
use ntest::timeout;

const HTML_COLORS: &[u8] = include_bytes!("./html_colors.txt");

#[test]
#[timeout(1000)]
fn test_word_red() {
    let word = b"red";

    let finder = ColorFinder::new();

    assert_eq!(
        finder.find(Slice::from(word, 0)),
        Some(("red".to_string(), 3))
    );
}

#[test]
#[timeout(1000)]
fn test_word_red_space() {
    let word = b"red ";

    let finder = ColorFinder::new();

    assert_eq!(
        finder.find(Slice::from(word, 0)),
        Some(("red".to_string(), 3))
    );
}

#[test]
#[timeout(1000)]
fn test_word_red_spaces() {
    let word = b" red\t ";

    let finder = ColorFinder::new();

    assert_eq!(finder.find(Slice::from(word, 0)), None);

    assert_eq!(
        finder.find(Slice::from(&word[1..], 1)),
        Some(("red".to_string(), 3))
    );
}

#[test]
#[timeout(1000)]
fn test_word_white_smoke() {
    let word = b"White \t Smoke \t ";

    let finder = ColorFinder::new();

    assert_eq!(
        finder.find(Slice::from(&word[..5], 0)),
        Some(("white".to_string(), 5))
    );
    assert_eq!(
        finder.find(Slice::from(&word[..10], 0)),
        Some(("white".to_string(), 5))
    );
    assert_eq!(
        finder.find(Slice::from(&word[..12], 0)),
        Some(("white".to_string(), 5))
    );
    assert_eq!(
        finder.find(Slice::from(&word[..], 0)),
        Some(("whitesmoke".to_string(), 13))
    );
}

#[test]
#[timeout(1000)]
fn test_all_colors() {
    let finder = ColorFinder::new();
    println!("size of data: {}", dbg!(mem::size_of::<ParsedAliasData>()));
}
