#![cfg(test)]

use super::*;

#[test]
fn iter1() {
    let v = MutMap::new();

    v.insert(22, 23);
    v.insert(44, 45);
    v.insert(66, 67);

    // Demonstrate iterating while also mutating the vector (in this
    // case, removing things from the end).
    let mut results = vec![];
    for i in v.iter() {
        results.push(Some(i));
    }

    assert_eq!(
        results,
        vec![Some((22, 23)), Some((44, 45)), Some((66, 67))]
    );
}
