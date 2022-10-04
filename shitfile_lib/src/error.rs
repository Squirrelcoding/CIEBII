use std::array::TryFromSliceError;

#[derive(thiserror::Error, Debug)]
pub enum ChunkError {
    #[error("A sequence of bytes of an invalid length was found.")]
    InvalidLen,
    #[error(
        "A checksum check has failed. This may mean that the data has been modified or corrupted."
    )]
    ChecksumFail,
    #[error("A illegal header was found in the file.")]
    IllegalHeader,
    #[error("An access to a non-existent chunk was attempted.")]
    NonExistentChunk,
    #[error("The dimensions do not correspond to the amount of chunks in the file.")]
    DimensionMismatch,

    #[error("Failed to parse bytes")]
    ByteParseFail(#[from] TryFromSliceError),
}
