
/// Creates a checksum given a stream of bytes
pub fn checksum(data: &[u8]) -> u16 {
    let mut total: u16 = 0;
    let mut prev: u8 = 0xAB;
    data.iter().for_each(|b| {

        // XOR the byte with the previous modified byte
        let new_byte = b ^ prev;

        // Add the new byte to the total
        total += new_byte as u16;

        // Modify the previous byte
        prev = new_byte - (total << 8) as u8;
    });

    // Return the total
    total
}
