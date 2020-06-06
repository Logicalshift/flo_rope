use crate::*;

use std::rc::*;
use std::cell::*;

#[test]
fn push_before_remove_middle() {
    let has_changed = Rc::new(RefCell::new(false));
    let set_changed = Rc::clone(&has_changed);

    let rope        = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let mut rope    = PushBeforeRope::from(rope, move |action| { assert!(action == &RopeAction::Replace(1..7, vec![])); (*set_changed.borrow_mut()) = true; });

    rope.replace(1..7, vec![]);

    assert!(*has_changed.borrow());
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
    assert!(rope.len() == 2);
}

#[test]
fn push_before_after_middle() {
    let has_changed = Rc::new(RefCell::new(false));
    let set_changed = Rc::clone(&has_changed);

    let rope        = AttributedRope::<_, ()>::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let mut rope    = PushAfterRope::from(rope, move |action| { assert!(action == RopeAction::Replace(1..7, vec![])); (*set_changed.borrow_mut()) = true; });

    rope.replace(1..7, vec![]);

    assert!(*has_changed.borrow());
    assert!(rope.read_cells(0..8).cloned().collect::<Vec<_>>() == vec![1, 8]);
    assert!(rope.len() == 2);
}
