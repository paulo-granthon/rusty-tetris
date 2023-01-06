use std::{fs::OpenOptions, io::{Write, Read}};


// private fn to write to binary
fn _write_binary <const S: usize> (path: &str, data: [u8; S], append: bool) -> Result<(), std::io::Error> {

    // open file with following permissions
    let mut f = OpenOptions::new()
        .create(true)
        .write(!append)
        .truncate(!append)
        .append(append)
        // .read(true)
        .open(format!("{}.bin", path))?;

    // append to the file and return the result
    match f.write(&data[..]) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    }
}

// writes a stream of bytes to the binary file at given path, replacing previous content
pub fn write_binary <const S: usize> (path: &str, data: [u8; S]) -> Result<(), std::io::Error> {
    _write_binary(path, data, false)
}

// appends a new stream of bytes to the binary file at given path
pub fn append_binary <const S: usize> (path: &str, data: [u8; S]) -> Result<(), std::io::Error> {
    _write_binary(path, data, true)
}

// loads the bytes of the file at given path
pub fn load_binary (path: &str) -> Result<Vec<u8>, std::io::Error> {

    // open file with following permissions
    let mut f = OpenOptions::new()
        .read(true)
        .open(format!("{}.bin", path))?;

    let num_bytes = f.metadata()?.len() as usize;

    // initialize a buffer for the file data
    let mut buffer = vec![0; num_bytes];

    // read the file and store on buffer
    f.read_to_end(&mut buffer)?;

    // println!("file_handler.load_binary -- path: {} | num_bytes: {} | buffer: {:?} | return: {:?}", path, num_bytes, buffer, &buffer[num_bytes..]);

    // skip 24 bytes from the beggining of the buffer and return the result
    Ok(buffer[num_bytes..].to_vec())
}
