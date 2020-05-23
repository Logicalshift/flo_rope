use super::rope_trait::*;
use super::rope_action::*;

///
/// A rope that can be edited by the user
///
pub trait RopeMut : Rope {
    ///
    /// Performs the specified editing action to this rope
    ///
    fn edit(&mut self, action: RopeAction<Self::Cell, Self::Attribute>);
}
