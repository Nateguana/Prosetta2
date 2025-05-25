#![cfg(test)]
use super::*;
use itertools::{self, Itertools};
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_word_red() {
    let word = b"red";

    let finder = ColorFinder::new();

    assert_eq!(finder.find(Slice::from(word, 0)), Some("red"));
}

#[test]
#[timeout(1000)]
fn test_word_red_space() {
    let word = b"red ";

    let finder = ColorFinder::new();

    assert_eq!(finder.find(Slice::from(word, 0)), Some("red"));
}

#[test]
#[timeout(1000)]
fn test_word_red_spaces() {
    let word = b" red\t ";

    let finder = ColorFinder::new();

    assert_eq!(finder.find(Slice::from(word, 0)), None);

    assert_eq!(
        finder.find(Slice::from(&word[1..], 1)),
        Some("red")
    );
}

#[test]
#[timeout(1000)]
fn test_word_white_smoke() {
    let word = b"White \t Smoke";

    let finder = ColorFinder::new();

    assert_eq!(
        finder.find(Slice::from(&word[..5], 0)),
        Some("white")
    );
    assert_eq!(
        finder.find(Slice::from(&word[..10], 0)),
        Some("white")
    );
    assert_eq!(
        finder.find(Slice::from(&word[..12], 0)),
        Some("white")
    );
    assert_eq!(
        finder.find(Slice::from(&word[..13], 0)),
        Some("white smoke")
    );
}
