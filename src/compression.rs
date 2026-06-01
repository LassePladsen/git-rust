pub type Compression = flate2::Compression;
pub type Decoder<R> = flate2::read::ZlibDecoder<R>;
pub type Encoder<R> = flate2::write::ZlibEncoder<R>;
