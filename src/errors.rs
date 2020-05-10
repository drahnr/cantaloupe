use thiserror::Error;
pub use anyhow::Result;

pub use anyhow::Context;

#[derive(Debug,Error)]
pub enum CantaError {
    #[error("Failed during IO operation: {0}")]
    IoError(std::io::Error),
    #[error("Failed to compress xml")]
    CompressionFailed,
    #[error("Unspecific error: {what}")]
    SomethingWentWrong {
        what: String,
    }
}



#[cfg(test)]
mod test {

}