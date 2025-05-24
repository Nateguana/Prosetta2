#![cfg(test)]
use super::*;
use itertools::{self, Itertools};
use ntest::timeout;

fn assert_add_letter(a: Vec<AliasParseData>, b: Vec<AliasName>) {
    let arr = a.into_iter().map(|e| e.alias).collect::<Vec<_>>();
    let good = itertools::equal(&arr, &b);
    assert!(
        good,
        "{:?} is not equal to {:?}",
        arr,
        b
    );
}

#[test]
#[timeout(1000)]
fn test_word_abcde() {
    let word = b"abcde";

    let mut finder_array = AliasFinderArray::new();

    for x in word.iter().cloned().combinations(3) {
        finder_array.add(x.try_into().unwrap());
    }

    let mut finder = AliasFinder::new(Arc::new(finder_array));

    assert_add_letter(finder.add_letter(word[0]), vec![]);
    assert_add_letter(finder.add_letter(word[1]), vec![]);
    assert_add_letter(finder.add_letter(word[2]), vec![*b"abc"]);
    assert_add_letter(finder.add_letter(word[3]), vec![*b"bcd", *b"abd", *b"acd"]);
    assert_add_letter(
        finder.add_letter(word[4]),
        vec![*b"cde", *b"bce", *b"bde", *b"abe", *b"ace", *b"ade"],
    );
}
