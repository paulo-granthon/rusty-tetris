use std::{fs::OpenOptions, io::{Write, Read}};

pub fn write_binary <const S: usize> (path: &str, data: [u8; S]) -> Result<(), std::io::Error> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(format!("{}.bin", path))?;

    // println!("The file: {:?}", f);
    // println!("Writing: {:?}", &data[..]);

    match f.write(&data[..]) {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    }
}

pub fn load_binary(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .open(format!("{}.bin", path))?;

    // println!("The file: {:?}", f);

    let num_bytes = f.metadata()?.len() as usize;

    // println!("Metadata: {:?} | num_bytes: {}", f.metadata(), num_bytes);

    let mut buffer = vec![0; num_bytes];

    // read up to 10 bytes
    // let read_result = 
    f.read_to_end(&mut buffer)?;
    // println!("Read Buffer: {:?}\nread_result: {}", &buffer[..], read_result);

    // let decoded: World = deserialize(&buffer[..]).unwrap();
    // println!("Decoded: {:?}", decoded);

    Ok(buffer[24..].to_vec())
}
