use crate::*;
use crate::rope::*;

#[test]
fn read_single_node() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn read_single_node_attributes() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    assert!(rope.read_attributes(0) == (&(), 0..8));
}

#[test]
fn read_single_node_attributes_from_middle() {
    let rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(rope.read_attributes(2) == (&(), 0..8));
}

#[test]
fn set_attributes() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.set_attributes(3..6, 2);

    assert!(rope.len() == 8);
    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..6));
    assert!(rope.read_attributes(6) == (&0, 6..8));
}

#[test]
fn set_attributes_on_neighboring_ranges() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3]);

    rope.set_attributes(1..2, 1);
    rope.set_attributes(2..3, 1);

    assert!(rope.len() == 3);
    assert!(rope.read_attributes(0) == (&0, 0..1));
    assert!(rope.read_attributes(1) == (&1, 1..3));
    assert!(rope.read_attributes(2) != (&0, 2..3));
    assert!(rope.read_attributes(2) == (&1, 2..3));
}

#[test]
fn set_attributes_overlapping_ranges() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6]);

    rope.set_attributes(1..3, 1);
    rope.set_attributes(2..4, 1);

    assert!(rope.len() == 6);
    assert!(rope.read_attributes(0) == (&0, 0..1));
    assert!(rope.read_attributes(1) == (&1, 1..4));
    assert!(rope.read_attributes(4) == (&0, 4..6));
}

#[test]
fn replace_attributes() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    rope.replace_attributes(3..6, vec![9, 10], 2);

    assert!(rope.len() == 7);
    assert!(rope.read_cells(0..rope.len()).cloned().collect::<Vec<_>>() == vec![1,2,3,9,10,7,8]);

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

    rope.split_at(4);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(1);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    assert!(rope.len() == 8);
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn read_after_full_split_and_join() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(4);
    rope.split_at(2);
    rope.split_at(3);
    rope.split_at(1);
    rope.split_at(5);
    rope.split_at(6);
    rope.split_at(7);

    rope.join_at(2);
    rope.join_at(1);
    rope.join_at(4);
    rope.join_at(6);
    rope.join_at(5);
    rope.join_at(7);

    assert!(rope.len() == 8);
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
    assert!(rope.len() == 2);
}

#[test]
fn join_after_partial_split() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.split_at(4);

    rope.replace(1..7, vec![]);

    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
    assert!(rope.len() == 2);
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
    assert!(rope.len() == 2);
}

#[test]
fn insert_and_find() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    rope.replace(4..4, vec![9, 10, 11, 12]);

    assert!(rope.read_cells(0..12).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8]);

    assert!(rope.read_cells(4..8).cloned().collect::<Vec<_>>() == vec![9, 10, 11, 12]);
    assert!(rope.read_cells(8..12).cloned().collect::<Vec<_>>() == vec![5, 6, 7, 8]);
    assert!(rope.read_cells(0..4).cloned().collect::<Vec<_>>() == vec![1, 2, 3, 4]);

    assert!(rope.len() == 12);
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

    assert!(rope.len() == 12);
}

#[test]
fn append_at_middle_of_attribute_range() {
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    rope.replace_attributes(3..6, vec![9, 10], 2);
    rope.replace(4..4, vec![11, 12]);

    assert!(rope.len() == 9);
    assert!(rope.read_cells(0..rope.len()).cloned().collect::<Vec<_>>() == vec![1,2,3,9,11,12,10,7,8]);

    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..7));
    assert!(rope.read_attributes(7) == (&0, 7..9));
}

#[test]
fn append_at_end_of_attribute_range() {
    // If replace() is called at a point that is between two ranges, we take the attributes from the left rather than the right
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    rope.replace_attributes(3..6, vec![9, 10], 2);
    rope.replace(5..5, vec![11, 12]);

    assert!(rope.len() == 9);
    assert!(rope.read_cells(0..rope.len()).cloned().collect::<Vec<_>>() == vec![1,2,3,9,10,11,12,7,8]);

    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..7));
    assert!(rope.read_attributes(7) == (&0, 7..9));
}

#[test]
fn replace_at_end_of_attribute_range() {
    // If replace() is called at a point that is between two ranges, we take the attributes from the left rather than the right
    let mut rope = AttributedRope::<_, i64>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);

    assert!(rope.len() == 8);
    rope.replace_attributes(3..6, vec![9, 10], 2);
    rope.replace(5..7, vec![11, 12]);

    assert!(rope.len() == 7);
    assert!(rope.read_cells(0..rope.len()).cloned().collect::<Vec<_>>() == vec![1,2,3,9,10,11,12]);

    assert!(rope.read_attributes(0) == (&0, 0..3));
    assert!(rope.read_attributes(3) == (&2, 3..7));
}

#[test]
fn utf_from_string() {
    let rope = AttributedRope::<_, ()>::from_str("Test");
    assert!(rope.to_string_lossy() == "Test".to_string());
}

#[test]
fn extend_rope() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3]);
    rope.extend(vec![4,5,6]);
    assert!(rope.read_cells(0..6).cloned().collect::<Vec<_>>() == vec![1,2,3,4,5,6]);
}

#[test]
fn extend_rope_references() {
    let mut rope = AttributedRope::<_, ()>::from(vec![1, 2, 3]);
    rope.extend(&vec![4,5,6]);
    assert!(rope.read_cells(0..6).cloned().collect::<Vec<_>>() == vec![1,2,3,4,5,6]);
}

#[test]
fn equal_cells() {
    let rope1 = AttributedRope::<_, ()>::from(vec![1, 2, 3]);
    let rope2 = AttributedRope::<_, ()>::from(vec![1, 2, 3]);

    assert!(rope1 == rope2);
}

#[test]
fn equal_attributes() {
    let mut rope1 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);
    let mut rope2 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);

    rope1.set_attributes(1..3, 1);
    rope2.set_attributes(1..3, 1);

    assert!(rope1 == rope2);
    assert!(rope2 == rope1);
}

#[test]
fn equal_attributes_ranges() {
    let mut rope1 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);
    let mut rope2 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);

    rope1.set_attributes(1..3, 1);
    rope2.set_attributes(1..2, 1);
    rope2.set_attributes(2..3, 1);

    assert!(rope1 == rope2);
    assert!(rope2 == rope1);
}

#[test]
fn unequal_cells() {
    let rope1 = AttributedRope::<_, ()>::from(vec![1, 2, 3]);
    let rope2 = AttributedRope::<_, ()>::from(vec![1, 2, 4]);

    assert!(rope1 != rope2);
    assert!(rope2 != rope1);
}

#[test]
fn unequal_length() {
    let rope1 = AttributedRope::<_, ()>::from(vec![1, 2, 3]);
    let rope2 = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4]);

    assert!(rope1 != rope2);
    assert!(rope2 != rope1);
}

#[test]
fn unequal_attribute_ranges() {
    let mut rope1 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);
    let mut rope2 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);

    rope1.set_attributes(1..3, 1);
    rope2.set_attributes(1..2, 1);

    assert!(rope1 != rope2);
    assert!(rope2 != rope1);
}

#[test]
fn unequal_attribute_values() {
    let mut rope1 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);
    let mut rope2 = AttributedRope::<_, i64>::from(vec![1, 2, 3]);

    rope1.set_attributes(1..3, 1);
    rope2.set_attributes(1..3, 2);

    assert!(rope1 != rope2);
    assert!(rope2 != rope1);
}
