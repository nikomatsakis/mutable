#![cfg(test)]

use super::*;

#[test]
fn iter1() {
    let v = MutVec::new();

    v.push(22);
    v.push(44);
    v.push(66);

    // Demonstrate iterating while also mutating the vector (in this
    // case, removing things from the end).
    let mut results = vec![];
    for i in v.iter() {
        results.push(Some(i));
        results.push(v.pop());
    }

    assert_eq!(results, vec![Some(22), Some(66), Some(44), Some(44)],);
}
