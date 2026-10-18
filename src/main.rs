use flate2::read::ZlibDecoder;
use std::io;
use std::io::{Read, Write};

fn main() {
    let mut stdin = io::stdin();
    let mut input = vec![];
    stdin
        .read_to_end(&mut input)
        .expect("cannot receive from stdin");

    let output = unzip(input.as_slice()).unwrap();

    let mut stdout = io::stdout();
    stdout
        .write_all(output.as_slice())
        .expect("cannot write to stdout");
}
fn unzip(content: &[u8]) -> io::Result<Vec<u8>> {
    let mut zlib = ZlibDecoder::new(content);
    let mut out = vec![];
    let _ = zlib.read_to_end(&mut out)?;
    Ok(out)
}
