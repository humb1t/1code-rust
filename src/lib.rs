mod de;
mod error;
pub mod test_struct;
mod size_cmp;
pub mod ser;

//pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
