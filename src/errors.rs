use crate::failure;
use crate::failure::Fail;

#[derive(Debug,Fail)]
pub enum CantaError {
    #[fail(display = "Failed during IO operation")]
    IoError,
    #[fail(display = "Failed to compress xml")]
    CompressionFailed,
    #[fail(display = "Unspecific error: {}", what)]
    SomethingWentWrong {
        what: String,
    }
}