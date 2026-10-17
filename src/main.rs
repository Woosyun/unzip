use flate2::read::ZlibDecoder;
use std::io::{Read, Write};
use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args();

    if args.len() != 2 {
        panic!("argument should be only one, oid.");
    }

    // receive oid
    let oid = args
        .skip(1) // pass program path
        .collect::<String>();
    let dir_name = oid.chars()
        .take(2)
        .collect::<String>();
    let file_name = oid.chars()
        .skip(2)
        .collect::<String>();

    // read content
    let current_dir = env::current_dir()?
        .ancestors()
        .find(|path| path.join(".git").exists())
        .expect("cannot find .git directory")
        .to_path_buf();
    
    let target_path = current_dir.join(".git").join("objects").join(dir_name).join(file_name);
    let content = fs::read(target_path)?;

    //decode
    let mut zlib = ZlibDecoder::new(&content[..]);
    let mut out = vec![];
    zlib.read_to_end(&mut out)?;

    //print
    std::io::stdout().write_all(&out[..])?;
    Ok(())
}
