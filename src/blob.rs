use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

#[derive(Debug)]
pub enum BlobError<'a> {
    Io(io::Error),
    Utf8Error(std::str::Utf8Error),
    NotABlob(&'a str),
}
pub type BlobResult<'a, T> = Result<T, BlobError<'a>>;

impl Display for BlobError<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BlobError::Io(err) => write!(f, "{}", err),
            BlobError::Utf8Error(err) => write!(f, "{}", err),
            BlobError::NotABlob(path) => write!(f, "Path '{}' is not a blob", path),
        }
    }
}

impl From<io::Error> for BlobError<'_> {
    fn from(err: io::Error) -> Self {
        BlobError::Io(err)
    }
}

impl From<std::str::Utf8Error> for BlobError<'_> {
    fn from(err: std::str::Utf8Error) -> Self {
        BlobError::Utf8Error(err)
    }
}

pub fn read_blob(path: &str) -> BlobResult<'_, Vec<u8>> {
    // Open file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    // TODO: Decompress the contents

    // Read file.
    let mut header = [0u8; 5];
    reader.read_exact(&mut header)?;

    // First assert its a blob by checking first 5 characters is "blob "
    if b"blob " != &header {
        return Err(BlobError::NotABlob(path));
    }

    // Read content byte length by reading to null-byte
    let mut content_len_bytes = Vec::new();
    reader.read_until(0, &mut content_len_bytes)?;
    // Drop the null-byte
    content_len_bytes.pop();

    // Convert length to integer
    let Ok(content_len) = str::from_utf8(&content_len_bytes)?.parse::<u8>() else {
        return Err(BlobError::NotABlob(path));
    };

    // Read contents with the found length
    let mut contents = vec![0u8; content_len.into()];
    let _ = reader.read_exact(&mut contents);
    Ok(contents)
}
