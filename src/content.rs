//! Content processing for POST and PUT.

use mime::Mime;

/// Processed content from PUT and POST requests.
///
/// This will automatically store posted content in its most sensible form based on its MIME type.
pub struct Content {
    mime: Mime,
    data: Data,
}

/// The stored form of the data.
enum Data {
    /// Encoded form data.
    Form(String),
    /// JSON encoded data.
    Json(String),
    /// XML encoded data.
    Xml(String),
    /// Other plaintext data.
    Text(String),
    /// Multipart encoded data.
    Multipart(Vec<Data>),
    /// Binary data stored in memory.
    Blob(Vec<u8>),
}
