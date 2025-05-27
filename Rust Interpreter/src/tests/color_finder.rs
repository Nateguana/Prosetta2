#![cfg(test)]
use std::{collections::HashSet, mem};

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
fn test_all_colors_match() {
    let finder = ColorFinder::new();

    let colors = HTML_COLORS.split(|&e| e == b',');

    for color in colors {
        let color_split: Vec<&[u8]> = color.split(|&e| e == b' ').collect::<Vec<_>>();
        let colors_result = String::from_utf8(color.replace(" ", "")).unwrap();
        for spaces in itertools::repeat_n([b"" as &[u8], b" ", b"   ", b"\t"], color_split.len())
            .multi_cartesian_product()
            .collect::<Vec<_>>()
        {
            let end_len = spaces.last().unwrap().len();
            let result: Vec<u8> = color_split
                .iter()
                .cloned()
                .interleave(spaces.into_iter())
                .flatten()
                .cloned()
                .collect();

            assert_eq!(
                finder.find(Slice::from(result.as_slice(), 0)),
                Some((colors_result.clone(), result.len() - end_len))
            );
        }
    }
}

#[test]
// #[ignore]
#[timeout(10000)]
fn test_all_colors_no_match() {
    fn get_set_color_len(set: &HashSet<&[u8]>, color: &Vec<&[u8]>) -> Option<usize> {
        for end in (1..=color.len()).rev() {
            let color = bstr::join(b" ", color[0..end].iter());
            if set.contains(color.as_slice()) {
                return Some(color.len());
            }
        }
        None
    }

    let finder = ColorFinder::new();

    let colors: Vec<&[u8]> = HTML_COLORS.split(|&e| e == b',').collect();

    let mut color_set = HashSet::new();

    for &color in &colors {
        color_set.insert(color);
    }

    let color_words: Vec<&[u8]> = colors
        .into_iter()
        .flat_map(|e| e.split(|&e| e == b' '))
        .unique()
        .collect();

    let color_word_set = (1..=3).flat_map(|len| {
        itertools::repeat_n(color_words.iter().cloned(), len).multi_cartesian_product()
    });

    let size: usize = color_word_set.clone().count();

    println!("size: {}", size);

    for (index, set) in color_word_set.enumerate() {
        let mut result = None;
        if let Some(length) = get_set_color_len(&color_set, &set) {
            result = Some(length);
        }
        let slice = bstr::join(b" ", set.iter());

        assert_eq!(
            finder.find(Slice::from(slice.as_slice(), 0)).map(|e| e.1),
            result,
            "{:?} failed",
            slice
        );

        if index % 10000 == 0 {
            eprintln!(
                "done {index}/{size} for {}%",
                index as f32 / size as f32 * 100.0
            )
        }
    }
}

#[test]
// #[ignore]
#[timeout(10000)]
fn test_all_colors_first_space() {
    let finder = ColorFinder::new();

    let colors = HTML_COLORS.split(|&e| e == b',');

    for color in colors {
        let mut vec: Vec<u8> = b" ".to_vec();
        vec.extend_from_slice(color);
        assert_eq!(finder.find(Slice::from(vec.as_slice(), 0)), None);
    }
}
