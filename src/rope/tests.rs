use crate::*;
use crate::rope::*;

#[test]
fn read_single_node() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn read_single_node_attributes() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(rope.read_attributes(0) == (&(), 0..8));
}

#[test]
fn set_attributes() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.set_attributes(3..6, 2);

    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..6));
    assert!(rope.read_attributes(6) == (&0, 6..8));
}

#[test]
fn replace_attributes() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.replace_attributes(3..6, vec![9, 10], 2);

    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..5));
    assert!(rope.read_attributes(6) == (&0, 5..7));
}

#[test]
fn read_mid_range() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(rope.read_cells(3..5).cloned().collect::<Vec<_>>() == vec![4, 5]);
}

#[test]
fn read_after_full_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(1);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(4);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn read_attributes_after_full_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(1);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(4);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    assert!(rope.read_attributes(0) == (&(), 0..8));
}

#[test]
fn remove_middle() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.replace(1..7, vec![]);

    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
}

#[test]
fn join_after_partial_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(4);

    rope.replace(1..7, vec![]);

    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
}

#[test]
fn join_after_full_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(1);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(4);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    rope.replace(1..7, vec![]);

    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
}

#[test]
fn insert_and_find() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.replace(4..4, vec![9, 10, 11, 12]);

    assert!(rope.read_cells(0..12).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8]);

    assert!(rope.read_cells(4..8).cloned().collect::<Vec<_>>() == vec![9, 10, 11, 12]);
    assert!(rope.read_cells(8..12).cloned().collect::<Vec<_>>() == vec![5, 6, 7, 8]);
    assert!(rope.read_cells(0..4).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4]);
}

#[test]
fn insert_and_find_after_full_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(1);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(4);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    rope.replace(4..4, vec![9, 10, 11, 12]);

    assert!(rope.read_cells(0..12).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8]);

    assert!(rope.read_cells(4..8).cloned().collect::<Vec<_>>() == vec![9, 10, 11, 12]);
    assert!(rope.read_cells(8..12).cloned().collect::<Vec<_>>() == vec![5, 6, 7, 8]);
    assert!(rope.read_cells(0..4).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4]);
}
