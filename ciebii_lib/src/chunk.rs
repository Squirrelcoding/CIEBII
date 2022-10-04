use super::checksum::checksum;
use super::error::*;
use super::rgb::RGB;

/// A ciebii chunk consisting of an RGB code along with a checksum
/// ```
/// use ciebii_lib::chunk::Chunk;
/// let chunk = Chunk::new(255, 0, 0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Chunk {
    rgb: RGB,
    checksum: u16,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let rgb = RGB::new(r, g, b);
        let checksum = checksum(&rgb.as_bytes());
        Self { rgb, checksum }
    }

    /// Returns the RGB code in a ciebii RGB struct.
    pub fn rgb(&self) -> RGB {
        self.rgb
    }

    /// Returns the u16 checksum of this chunk.
    pub fn checksum(&self) -> u16 {
        self.checksum
    }

    /// Returns this chunk as a vector of bytes.
    /// It returns it in the format \[RGB | CHECKSUM]
    pub fn as_bytes(&self) -> Vec<u8> {

        // Merge the rgb and checksum
        let bytes: Vec<u8> = self
            .rgb
            .as_bytes()
            .iter()
            .chain(self.checksum.to_be_bytes().iter())
            .cloned()
            .collect();
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {

        // All chunks need to be 5 bytes
        if bytes.len() != 5 {
            return Err(ChunkError::InvalidLen);
        }

        // Seperate the RGB and checksum
        let (rgb, check) = bytes.split_at(3);

        // calculate the new checksum on the given RGB
        let new_checksum = checksum(rgb);

        // create an RGB struct
        let rgb = RGB::new(rgb[0], rgb[1], rgb[2]);

        // Do some bit shifting to get the original checksum
        let original_checksum = ((check[0] as u16) << 8) | check[1] as u16;

        // Compare the checksums
        if original_checksum != new_checksum {
            return Err(ChunkError::ChecksumFail);
        }

        Ok(Chunk {
            rgb,
            checksum: new_checksum,
        })
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;

    #[test]
    fn create_new_chunk() {
        let chunk = Chunk::new(255, 0, 0);
        assert_eq!(chunk.rgb(), RGB::new(255, 0, 0));
        assert_eq!(chunk.checksum, 252);
        assert_eq!(chunk.as_bytes(), [255, 0, 0, 0, 252]);
    }

    #[test]
    fn try_from_invalid_len() {
        let data: Vec<u8> = vec![1, 2, 3];

        let chunk = Chunk::try_from(&data[..]);

        assert!(chunk.is_err());

        if let ChunkError::InvalidLen = chunk.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn try_from_checksum_fail() {
        let data: Vec<u8> = vec![171, 205, 239, 255, 239];

        let chunk = Chunk::try_from(&data[..]);

        assert!(chunk.is_err());

        if let ChunkError::ChecksumFail = chunk.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn try_from_successfully() {
        let data: Vec<u8> = vec![171, 205, 239, 0, 239];

        let chunk = Chunk::try_from(&data[..]);

        assert!(chunk.is_ok());

        let chunk = chunk.unwrap();

        assert_eq!(chunk.rgb(), RGB::new(0xAB, 0xCD, 0xEF));
    }
}
