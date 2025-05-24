#![cfg(test)]
use super::*;
use itertools::{self, Itertools};
use ntest::timeout;

fn assert_add_letter(a: Vec<AliasParseData>, b: Vec<AliasName>) {
    let a_alias = a.iter().map(|e| e.alias).collect::<Vec<_>>();
    let b_alias = &b;

    let good1 = itertools::equal(&a_alias, &b);
    assert!(good1, "{:?} is not equal to {:?}", &a_alias, &b_alias);

    let a_pos = a.iter().map(|e| e.pos).collect::<Vec<_>>();
    let b_pos = b
        .iter()
        .map(|e| -> [u8; 3] {
            e.iter()
                .map(|e| e - b'a')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let good2 = itertools::equal(&a_pos, &b_pos);
    assert!(good2, "{:?} is not equal to {:?}", &a_pos, &b_pos);
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

    // all aliases are used -- nothing left
    for j in 0..5 {
        assert_add_letter(finder.add_letter(word[j]), vec![]);
    }
}
