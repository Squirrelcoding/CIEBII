use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
/// A struct representing an RGB color
/// ```
/// use ciebii_lib::rgb::RGB;
/// let rgb = RGB::new(0xFF, 0xFF, 0xFF);
/// ```
pub struct RGB(u8, u8, u8);

impl RGB {

    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Get the color
    pub fn color(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Returns this RGB struct as a 3-byte-long slice.
    pub fn as_bytes(&self) -> Vec<u8> {

        // Bytes of R, bytes of G and bytes of B
        let bytes = self
            .0
            .to_be_bytes()
            .iter()
            .chain(self.1.to_be_bytes().iter())
            .chain(self.2.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>();

        bytes
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.0, self.1, self.2)
    }
}
