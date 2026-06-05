use crate::{compression::{Compression, Encoder}, object};
use sha1::{Digest, Sha1};
use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
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
    let contents = fs::read_to_string(path);
    println!("LP contents: {contents:?}");
    let header = "";
    object::Object::read(path);

    // First assert its a blob by checking first 5 characters is "blob "
    // if "blob " != header {
    //     return Err(BlobError::NotABlob(path));
    // }

    // // Read content byte length by reading to null-byte
    // let mut content_len_bytes = Vec::new();
    // reader.read_until(0, &mut content_len_bytes)?;
    // // Drop the null-byte
    // content_len_bytes.pop();

    // // Convert length to integer
    // let Ok(content_len) = str::from_utf8(&content_len_bytes)?.parse::<u8>() else {
    //     return Err(BlobError::NotABlob(path));
    // };

    // // Read contents with the found length
    // let mut contents = vec![0u8; content_len.into()];
    // let _ = reader.read_exact(&mut contents);
    // Ok(contents)
    //
    Ok(Vec::new())
}

pub fn write_blob(bytes: &[u8]) -> BlobResult<'_, ()> {
    // Write blob header
    let length = bytes.len();
    let header = format!("blob {length}\0");
    let blob_bytes: Vec<u8> = [header.as_bytes(), bytes].concat();

    // Hash blob
    let hash = hex::encode(Sha1::digest(&blob_bytes));
    let hash_bytes = hash.as_bytes(); // idk why i need to bytes -> str -> bytes...

    // Write to stdout
    let _ = io::stdout().write_all(hash_bytes);
    println!(); // newline

    // Compress
    let mut encoder = Encoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob_bytes)?;
    let compressed = encoder.finish()?;

    // Find path to objects dir. path e3123456 is .git/objects/e3/123456
    let dir_path = format!(".git/objects/{}", &hash[0..2]);
    let _ = fs::create_dir(&dir_path);
    let file_path = format!("{dir_path}/{}", &hash[2..]);
    let mut file = File::create(file_path).expect("Could not create file");

    // Write blob
    file.write_all(&compressed)?;
    Ok(())
}
