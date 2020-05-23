///
/// Represents a read-only Rope data structure
///
pub trait Rope : Clone {
    /// A 'cell' or character in the rope. For a UTF-8 rope this could be `u8`, for xample
    type Cell: Clone;

    /// The type of an attribute in the rope. Every cell range has an attribute attached to it
    type Attribute: Clone+Default;
}
