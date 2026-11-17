use glib::prelude::*;
use glib::Variant;

fn strings(values: &[&str]) -> Variant { values.to_variant() }

#[test]
fn forward_unicode_and_reverse() {
    let values = ["", "α", "東京", "🙂", "tail"];
    let v = strings(&values);
    assert_eq!(v.array_iter_str().unwrap().collect::<Vec<_>>(), values);
    assert_eq!(v.array_iter_str().unwrap().rev().collect::<Vec<_>>(), values.into_iter().rev().collect::<Vec<_>>());
}

#[test]
fn interleaved_front_back_preserves_order_and_exhaustion() {
    let v = strings(&["zero", "one", "two", "three", "four"]);
    let mut it = v.array_iter_str().unwrap();
    assert_eq!(it.next(), Some("zero"));
    assert_eq!(it.next_back(), Some("four"));
    assert_eq!(it.nth(1), Some("two"));
    assert_eq!(it.len(), 1);
    assert_eq!(it.next_back(), Some("three"));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
    assert_eq!(it.len(), 0);
}

#[test]
fn nth_back_and_last() {
    let v = strings(&["0", "1", "2", "3", "4", "5"]);
    let mut it = v.array_iter_str().unwrap();
    assert_eq!(it.nth_back(1), Some("4"));
    assert_eq!(it.next_back(), Some("3"));
    assert_eq!(it.last(), Some("2"));
}

#[test]
fn empty_and_singleton_are_fused() {
    let empty = strings(&[]);
    let mut it = empty.array_iter_str().unwrap();
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
    let single = strings(&["only"]);
    let mut it = single.array_iter_str().unwrap();
    assert_eq!(it.next_back(), Some("only"));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn nth_overflow_exhausts_without_dereferencing() {
    let v = strings(&["first", "second"]);
    let mut it = v.array_iter_str().unwrap();
    assert_eq!(it.nth(usize::MAX), None);
    assert_eq!(it.next_back(), None);
    let mut it = v.array_iter_str().unwrap();
    assert_eq!(it.nth_back(usize::MAX), None);
    assert_eq!(it.next(), None);
}
