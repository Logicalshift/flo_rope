use crate::api::*;

///
/// A pull rope will notify its function when changes are available and will gather changes into
/// a single batch when they're 'pulled' from the rope. This is useful in circumstances where
/// updates are scheduled but not performed immediately, for example when updating a UI. Pulling
/// changes only when the UI is ready to redraw will reduce the number of updates required to
/// end up with a representation of the most recent state of the rope.
///
pub struct PullRope<BaseRope, PullFn> 
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
    /// The rope that this will pull changes from
    rope: BaseRope,

    /// A function that is called whenever the state of this rope changes from 'no changes' to 'changes waiting to be pulled'
    pull_fn: PullFn
}

impl<BaseRope, PullFn> PullRope<BaseRope, PullFn>
where 
BaseRope:   RopeMut, 
PullFn:     Fn() -> () {
    ///
    /// Creates a new pull rope from a base rope and a pull function
    /// 
    /// The base rope is used as storage for this pull rope, and the pull function is called whenever the state of
    /// the rope changes from 'no changes' to 'changes waiting to be pulled'
    ///
    pub fn from(rope: BaseRope, pull_fn: PullFn) -> PullRope<BaseRope, PullFn> {
        PullRope {
            rope:       rope,
            pull_fn:    pull_fn
        }
    }
}
