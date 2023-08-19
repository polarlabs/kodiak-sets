#![forbid(unsafe_code)]
//#![deny(missing_docs)]
// Lints for rustdoc
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
//#![deny(missing_doc_code_examples)] (unstable)
#![deny(rustdoc::invalid_codeblock_attributes)]
//#![deny(rustdoc::invalid_html_tags)] (unstable)
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::bare_urls)]
#![warn(rustdoc::private_doc_tests)]
/*
Collection of useful rustdoc options awaiting their implementation.
#![doc(html_logo_url = "https://example.com/logo.jpg")]
#![doc(html_favicon_url = "https://example.com/favicon.ico")]
*/
#![deny(clippy::all)]
#![deny(warnings)]

//! Get things organized with these powerful, yet easy to use sets. For the time being,
//! `kodiak-sets` offers a `Sequence`, which is an ordered sets of elements, with each
//! element at a unique, distinguishable position.

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
