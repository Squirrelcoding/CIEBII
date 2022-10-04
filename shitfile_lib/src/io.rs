use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{Context, Error};

use super::file::SHITFILE;


/// Tries to create a `SHITFILE` from `path`
/// 
/// # Example
/// 
/// ```no_run
/// use shitfile_lib::io::read_file;
/// use std::path::Path;
/// let path = Path::new("my_file.shf");
/// let file = read_file(&path);
/// ```
/// 
pub fn read_file(path: &Path) -> Result<SHITFILE, Error> {

    // try to open the file
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .with_context(|| format!("Failed to open file '{:?}'", path))?;

    // Get the metadata for the file length
    let metadata = fs::metadata(path)?;

    let mut vec = vec![0; metadata.len() as usize];

    // read the file into a vec
    file.read(&mut vec)?;

    match SHITFILE::try_from(vec) {
        Ok(file) => Ok(file),
        Err(err) => Err(err.into()),
    }
}


/// Attemps to write a `SHITFILE` to a file.
/// 
/// ```no_run
/// use shitfile_lib::io::write_file;
/// use shitfile_lib::file::SHITFILE;
/// use std::path::Path;
/// let path = Path::new("my_file.shf");
/// let shitfile = SHITFILE::new(2, 2);
/// let file = write_file(&path, &shitfile);
/// ```
/// 
pub fn write_file(path: &Path, shitfile: &SHITFILE) -> anyhow::Result<()> {

    // open file
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;

    // try to write to the file
    file.write_all(&shitfile.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod file_tests {
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Write},
    };

    use tempdir::TempDir;

    use crate::{chunk::Chunk, file::SHITFILE};

    fn test_file() -> SHITFILE {
        let chunks = vec![
            Chunk::new(0xAB, 0xCD, 0xEF),
            Chunk::new(0x12, 0x34, 0x56),
            Chunk::new(0x69, 0x42, 0x00),
            Chunk::new(0xDE, 0xAD, 0xA5),
        ];

        SHITFILE::try_from_chunks(2, 2, chunks).unwrap()
    }

    #[test]
    fn test_io() {
        let dir = TempDir::new("tests").unwrap();
        let file = dir.path().join("testfile.shf");
        let mut f = File::create(&file).unwrap();

        let test_file = test_file();

        assert!(f.write_all(&test_file.as_bytes()).is_ok());

        let shitfile = OpenOptions::new().read(true).open(&file);

        assert!(shitfile.is_ok());

        let mut shitfile = shitfile.unwrap();
        let metadata = std::fs::metadata(&file).unwrap();

        let mut vec = vec![0; metadata.len() as usize];

        assert!(shitfile.read(&mut vec).is_ok());

        let shitfile = SHITFILE::try_from(vec);

        assert!(shitfile.is_ok());

        let shitfile = shitfile.unwrap();

        assert_eq!(shitfile.dimensions(), (2, 2));
        assert_eq!(
            shitfile.chunks(),
            &vec![
                Chunk::new(0xAB, 0xCD, 0xEF),
                Chunk::new(0x12, 0x34, 0x56),
                Chunk::new(0x69, 0x42, 0x00),
                Chunk::new(0xDE, 0xAD, 0xA5),
            ]
        );

        assert_eq!(
            shitfile.as_bytes(),
            vec![
                83, 72, 73, 84, 70, 73, 76, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 10, 160, 171, 205, 239, 0, 239, 18, 52, 86, 2, 33, 105, 66, 0, 1, 194,
                222, 173, 165, 1, 202
            ]
        );
    }
}
