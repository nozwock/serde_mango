//#![deny(missing_docs)]
// #![doc(html_root_url = "http://arcnmx.github.io/serde-ini/")]

//! Windows INI format serialization for serde

pub mod de;
pub mod error;
pub mod ser;

pub use de::{from_bufread, from_read, from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, to_vec, to_writer, Serializer};

pub mod parse;
pub mod write;

pub use parse::{Item, Parser};
pub use write::{LineEnding, Writer};
