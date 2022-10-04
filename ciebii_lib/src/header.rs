use super::{checksum::checksum, error::ChunkError, file::CIEBIIFILE};

/// A header chunk consisting of 3 chunks. It contains the dimensions of the file and a checksum of the dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    x: usize,
    y: usize,
    checksum: u32,
}

#[allow(dead_code)]
impl Header {

    // Magic bytes spelling "CIEBIIFILE"
    //! TODO
    const MAGIC_BYTES: [u8; 10] = [67, 73, 69, 66, 73, 73, 70, 73, 76, 69];

    pub fn new(x: usize, y: usize) -> Self {

        // Merge the bytes of x and y to use them to create a checksum.
        let bytes: Vec<u8> = x
            .to_be_bytes()
            .iter()
            .chain(y.to_be_bytes().iter())
            .cloned()
            .collect();

        let checksum = checksum(&bytes) as u32;

        Self { x, y, checksum }
    }

    /// Returns the checksum of this header
    pub fn checksum(&self) -> u32 {
        self.checksum
    }

    /// Returns the dimensions of this header
    pub fn dimensions(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    // Returns the bytes as [HEADER, X (usize), Y (usize), CHECKSUM ]
    //                         10b     8b         8b          4b
    /// Returns the header as a byte array.
    pub fn as_bytes(&self) -> Vec<u8> {

        // magic bytes, then x, then y, then the checksum.
        Header::MAGIC_BYTES
            .iter()
            .chain(self.x.to_be_bytes().iter())
            .chain(self.y.to_be_bytes().iter())
            .chain(self.checksum.to_be_bytes().iter())
            .cloned()
            .collect()
    }
}

impl TryFrom<Vec<u8>> for Header {
    type Error = ChunkError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {

        // All headers must be 30 bytes long
        if bytes.len() != 30 {
            return Err(ChunkError::InvalidLen);
        }

        // The magic bytes
        let header = &bytes[0..10];

        // Make sure that the magic bytes match
        if header != CIEBIIFILE::MAGIC_BYTES {
            return Err(ChunkError::IllegalHeader);
        }

        // bytes for width
        let x = &bytes[10..18];

        // bytes for height
        let y = &bytes[18..26];

        // Original checksum input
        let old_checksum_data = &bytes[26..30];

        // New checksum input
        let new_checksum_data: Vec<u8> = x.iter().chain(y.iter()).cloned().collect();

        // try to create X from bytes
        let x = usize::from_be_bytes(x.try_into()?);

        // try to create Y from bytes
        let y = usize::from_be_bytes(y.try_into()?);

        let old_checksum = u32::from_be_bytes(old_checksum_data.try_into()?);

        let new_checksum = checksum(&new_checksum_data) as u32;

        // Compare the checksums
        if old_checksum != new_checksum {
            return Err(ChunkError::ChecksumFail);
        }

        Ok(Self {
            x,
            y,
            checksum: new_checksum,
        })
    }
}

#[cfg(test)]
mod header_tests {
    use super::*;

    fn create_header() -> Header {
        Header::new(20, 20)
    }

    #[test]
    fn create_header_test() {
        let header = create_header();

        assert_eq!(header.x, 20);
        assert_eq!(header.y, 20);
        assert_eq!(header.dimensions(), (20, 20));
        assert_eq!(header.checksum, 2896);
    }

    #[test]
    fn test_as_bytes() {
        let header = create_header();
        assert_eq!(
            header.as_bytes(),
            [
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
                20, 0, 0, 11, 80
            ]
        );
    }

    #[test]
    fn test_from_bytes_invalid_len() {
        let data = vec![1, 2, 3];

        let header = Header::try_from(data);

        assert!(header.is_err());

        if let ChunkError::InvalidLen = header.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_bytes_invalid_magic_bytes() {
        let bytes = vec![
            67, 73, 69, 00, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 20,
            0, 0, 11, 80,
        ];

        let header = Header::try_from(bytes);

        // assert!(header.is_err());

        if let ChunkError::IllegalHeader = header.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_bytes_checksum_fail() {
        let bytes = vec![
            67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
            20, 0, 0, 11, 80,
        ];

        let header = Header::try_from(bytes);

        assert!(header.is_err());

        if let ChunkError::ChecksumFail = header.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_bytes_successful() {
        let bytes = vec![
            67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 20,
            0, 0, 11, 80,
        ];

        let header = Header::try_from(bytes);

        assert!(header.is_ok());
        let header = header.unwrap();

        assert_eq!(header.checksum, 2896);
        assert_eq!(header.x, 20);
        assert_eq!(header.y, 20);
        assert_eq!(header.dimensions(), (20, 20));
    }
}
