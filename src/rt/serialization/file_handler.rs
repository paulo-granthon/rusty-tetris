use std::{fs::OpenOptions, io::{Write, Read}};

// formats the given profile to binary
pub fn to_bytes<const S: usize> (data: String, align_right: bool) -> Result<[u8; S], std::io::Error> {

    // get the name as bytes
    let name_bytes = data.to_owned().into_bytes();

    // initialize a S length u8 array
    let mut bytes: [u8; S] = [0; S];

    // loop trough the string bytes
    for i in 0..name_bytes.len() {
        bytes[if align_right {16 - name_bytes.len() + i} else { i }] = name_bytes[i];
    }

    // return the result
    Ok(bytes)
}

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

pub fn clear_binary (path: &str) -> Result<(), std::io::Error> {
    _write_binary(path, [], false)
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
