use crate::*;
use crate::rope::*;

#[test]
fn read_single_node() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 5, 6, 7, 8]);
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
