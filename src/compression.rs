use std::io::prelude::*;
use flate2::read::ZlibDecoder;

pub type Decoder<R> = ZlibDecoder<R>;
