use super::{chunk::Chunk, error::ChunkError, header::Header};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct representing an actual CIEBIIFILE.
/// ```
/// use ciebii_lib::file::CIEBIIFILE;
/// let file = CIEBIIFILE::new(20, 20);
/// ```
pub struct CIEBIIFILE {
    chunks: Vec<Chunk>,
    bytes: Vec<u8>,
    header: Header,
}

#[allow(dead_code)]
impl CIEBIIFILE {

    // Magic bytes spelling "CIEBIIFILE"

    //! TODO: CHANGE
    pub const MAGIC_BYTES: [u8; 10] = [67, 73, 69, 66, 73, 73, 70, 73, 76, 69];

    /// Create a new empty CIEBIIFILE
    pub fn new(x: usize, y: usize) -> CIEBIIFILE {

        // Create a header
        let header = Header::new(x, y);
        Self {
            chunks: Vec::new(),
            bytes: Vec::new(),
            header,
        }
    }

    /// Attemps to construct a CIEBIIFILE given a stream of chunks along with some dimensions
    pub fn try_from_chunks(x: usize, y: usize, chunks: Vec<Chunk>) -> Result<Self, ChunkError> {

        // See if the dimensions correspond the amount of given chunks
        if (x * y) != chunks.len() {
            return Err(ChunkError::DimensionMismatch);
        }

        let header = Header::new(x, y);

        // The actual payload of the chunks.
        let bytes: Vec<u8> = chunks.iter().flat_map(|chunk| chunk.as_bytes()).collect();

        Ok(Self {
            chunks,
            bytes,
            header,
        })
    }

    /// Returns the dimensions of the file
    pub fn dimensions(&self) -> (usize, usize) {
        self.header.dimensions()
    }

    /// Pushes a chunk and its bytes
    pub fn push_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
        self.bytes.append(&mut chunk.as_bytes());
    }

    /// Returns the chunks in a vec
    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    /// Turns this file into a raw byte format.
    pub fn as_bytes(&self) -> Vec<u8> {

        // Header, then bytes.
        self.header
            .as_bytes()
            .iter()
            .chain(self.bytes.iter())
            .cloned()
            .collect()
    }

    /// Remove a chunk at a given index
    pub fn remove_at_index(&mut self, index: usize) -> Result<Chunk, ChunkError> {

        // Check if the index is even valid
        if index >= self.chunks.len() {
            return Err(ChunkError::NonExistentChunk);
        }

        let removed = self.chunks.remove(index);

        // Update the bytes
        self.bytes = self
            .chunks
            .iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        Ok(removed)
    }

    /// Get a chunk at a given index
    pub fn get_at_index(&self, index: usize) -> Option<&Chunk> {
        self.chunks.get(index)
    }

    /// Modify a chunk at a given index
    pub fn modify(&mut self, index: usize, new_chunk: Chunk) -> Result<(), ChunkError> {

        // Check if the index is even valid.
        if index >= self.chunks.len() {
            return Err(ChunkError::NonExistentChunk);
        }

        // Set the new chunk
        self.chunks[index] = new_chunk;

        // Update the bytes
        self.bytes = self
            .chunks
            .iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        Ok(())
    }
}

impl TryFrom<Vec<u8>> for CIEBIIFILE {
    type Error = ChunkError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {

        // The header is always the first 30 bytes
        let header = &bytes[0..30];

        // Try to construct a header
        let header = Header::try_from(header.to_owned())?;

        let dimensions = header.dimensions();



        // Cant use iterators :(
        let mut chunks = Vec::new();        

        for chunk in bytes.chunks(5).skip(6) {
            chunks.push(Chunk::try_from(chunk)?);
        }

        // Verify that the length corresponds to the amount of chunks
        if chunks.len() != dimensions.0 * dimensions.1 {
            return Err(ChunkError::DimensionMismatch);
        }

        Ok(Self {
            chunks,
            bytes: bytes[30..].to_vec(),
            header,
        })
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn create_file() {
        let file = CIEBIIFILE::new(20, 20);

        assert_eq!(file.header.dimensions(), (20, 20));
        assert_eq!(file.chunks.len(), 0);
        assert_eq!(file.bytes.len(), 0);
    }

    #[test]
    fn push_chunk() {
        let mut file = CIEBIIFILE::new(20, 20);
        let chunk = Chunk::new(0xAB, 0xCD, 0xEF);
        file.push_chunk(chunk);

        assert_eq!(file.chunks.len(), 1);
        assert_eq!(file.bytes.len(), 5);
    }

    #[test]
    fn get_chunks() {
        let mut file = CIEBIIFILE::new(20, 20);
        let chunk = Chunk::new(0xAB, 0xCD, 0xEF);
        file.push_chunk(chunk);
        let chunk_clone = Chunk::new(0xAB, 0xCD, 0xEF);

        assert_eq!(file.chunks(), &vec![chunk_clone]);
    }

    #[test]
    fn as_bytes() {
        let mut file = CIEBIIFILE::new(20, 20);
        let chunk = Chunk::new(0xFF, 0x00, 0x00);
        file.push_chunk(chunk);

        assert_eq!(
            file.as_bytes(),
            [
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
                20, 0, 0, 11, 80, 255, 0, 0, 0, 252
            ]
        );
    }

    #[test]
    fn remove_at_index() {
        let mut file = CIEBIIFILE::new(20, 20);
        file.push_chunk(Chunk::new(0x69, 0x42, 0x00));
        file.push_chunk(Chunk::new(0xAB, 0xCD, 0xEF));
        file.push_chunk(Chunk::new(0x12, 0x34, 0x56));

        let removed = file.remove_at_index(1);

        assert!(removed.is_ok());
        let removed = removed.unwrap();

        assert_eq!(removed, Chunk::new(0xAB, 0xCD, 0xEF));
        assert_eq!(
            file.chunks,
            vec![Chunk::new(0x69, 0x42, 0x00), Chunk::new(0x12, 0x34, 0x56)]
        );
        assert_eq!(
            file.as_bytes(),
            [
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
                20, 0, 0, 11, 80, 105, 66, 0, 1, 194, 18, 52, 86, 2, 33
            ]
        );
    }

    #[test]
    fn get_at_index() {
        let mut file = CIEBIIFILE::new(20, 20);
        file.push_chunk(Chunk::new(0x69, 0x42, 0x00));
        file.push_chunk(Chunk::new(0xAB, 0xCD, 0xEF));
        file.push_chunk(Chunk::new(0x12, 0x34, 0x56));

        assert_eq!(file.get_at_index(0).unwrap(), &Chunk::new(0x69, 0x42, 0x00));
        assert_eq!(file.get_at_index(1).unwrap(), &Chunk::new(0xAB, 0xCD, 0xEF));
        assert_eq!(file.get_at_index(2).unwrap(), &Chunk::new(0x12, 0x34, 0x56));
    }

    #[test]
    fn modify_chunk() {
        let mut file = CIEBIIFILE::new(20, 20);
        file.push_chunk(Chunk::new(0x69, 0x42, 0x00));
        file.push_chunk(Chunk::new(0xAB, 0xCD, 0xEF));
        file.push_chunk(Chunk::new(0x12, 0x34, 0x56));

        assert!(file.modify(0, Chunk::new(1, 2, 3)).is_ok());
        assert_eq!(
            file.chunks,
            vec![
                Chunk::new(1, 2, 3),
                Chunk::new(0xAB, 0xCD, 0xEF),
                Chunk::new(0x12, 0x34, 0x56)
            ]
        );

        assert_eq!(
            file.as_bytes(),
            [
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
                20, 0, 0, 11, 80, 1, 2, 3, 1, 253, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33
            ]
        );
    }

    #[test]
    fn test_from_bytes_invalid_header() {
        let bytes = vec![
            123, 72, 73, 84, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0,
            20, 0, 0, 11, 80, 1, 2, 3, 1, 253, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33,
        ];

        let file = CIEBIIFILE::try_from(bytes);

        assert!(file.is_err());

        if let ChunkError::IllegalHeader = file.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_bytes_invalid_chunk() {
        let bytes = vec![
            67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 20,
            0, 0, 11, 80, 1, 2, 3, 1, 253, 171, 205, 239, 0, 239, 18, 52, 86, 20, 33,
        ];

        let file = CIEBIIFILE::try_from(bytes);

        assert!(file.is_err());

        if let ChunkError::ChecksumFail = file.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_bytes_successfully() {
        let bytes = vec![
            67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2,
            0, 0, 10, 160, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33, 222, 173, 190, 1, 179, 105,
            66, 50, 1, 244,
        ];

        let file = CIEBIIFILE::try_from(bytes);

        assert!(file.is_ok());

        let file = file.unwrap();

        assert_eq!(file.dimensions(), (2, 2));

        assert_eq!(
            file.chunks,
            vec![
                Chunk::new(0xAB, 0xCD, 0xEF),
                Chunk::new(0x12, 0x34, 0x56),
                Chunk::new(0xDE, 0xAD, 0xBE),
                Chunk::new(0x69, 0x42, 0x32),
            ]
        );

        assert_eq!(
            file.as_bytes(),
            vec![
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 10, 160, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33, 222, 173, 190, 1, 179,
                105, 66, 50, 1, 244,
            ]
        );

    }

    #[test]
    fn test_from_chunks_invalid_dimensions() {
        let chunks = vec![
            Chunk::new(0xAB, 0xCD, 0xEF),
            Chunk::new(0x12, 0x34, 0x56),
            Chunk::new(0x69, 0x42, 0x00),
            Chunk::new(0xDE, 0xAD, 0xA5),
        ];

        let file = CIEBIIFILE::try_from_chunks(20, 20, chunks);

        assert!(file.is_err());

        if let ChunkError::DimensionMismatch = file.unwrap_err() {
        } else {
            panic!()
        }
    }

    #[test]
    fn test_from_chunks_successfully() {
        let chunks = vec![
            Chunk::new(0xAB, 0xCD, 0xEF),
            Chunk::new(0x12, 0x34, 0x56),
            Chunk::new(0x69, 0x42, 0x00),
            Chunk::new(0xDE, 0xAD, 0xA5),
        ];

        let file = CIEBIIFILE::try_from_chunks(2, 2, chunks);

        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(file.dimensions(), (2, 2));
        assert_eq!(
            file.as_bytes(),
            [
                67, 73, 69, 66, 73, 73, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 10, 160, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33, 105, 66, 0, 1, 194,
                222, 173, 165, 1, 202
            ]
        );
    }
}
