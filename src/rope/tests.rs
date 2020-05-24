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
