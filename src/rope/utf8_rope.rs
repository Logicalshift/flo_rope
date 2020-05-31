use crate::api::*;

///
/// Trait implemented by attributed ropes that can work as strings
///
pub trait Utf8Rope {
    ///
    /// Creates a rope from a string
    ///
    fn from_str(string: &str) -> Self;

    ///
    /// Converts this rope to a string
    ///
    fn to_string_lossy(&self) -> String;
}

impl<R: Default+RopeMut<Cell=u8>> Utf8Rope for R {
    ///
    /// Creates a rope containing a string value
    ///
    fn from_str(string: &str) -> Self {
        let mut new_rope = Self::default();
        new_rope.replace(0..0, string.bytes());

        new_rope
    }

    ///
    /// Converts this rope to a string
    ///
    fn to_string_lossy(&self) -> String {
        // Generate a vec of all the bytes in this rope
        let bytes = self.read_cells(0..self.len())
            .map(|byte| *byte)
            .collect::<Vec<_>>();

        // Convert to string
        String::from_utf8_lossy(&bytes).into()
    }
}