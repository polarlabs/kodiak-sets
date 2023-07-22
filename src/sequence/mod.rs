//! `Sequence` is designed for use cases in which the elements' order has to get persisted
//! but the chosen persistence layer requires metadata to do so, e.g. a database.
//! `Sequence` allows to add and remove elements at any position, virtually infinitely.

// Keep crate's module structure completely private, see public re-exports below.
// (also hides modules from crate documentation)
mod sequence;

// Re-exports for convenient use within crate.
// none

// Publicly re-exporting all items valuable to users.
// (avoids explicitly listing re-exports in crate documentation as there is no alternate path to those items)
pub use sequence::Node;
pub use sequence::Pos;
pub use sequence::Sequence;
