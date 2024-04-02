use std::{io::{Error, Read, Write}, net::TcpStream};

pub fn send_headered(data: &[u8], stream: &mut TcpStream) -> Result<(), Error> {

    let data_length = data.len() as u64;

    match stream.write_all(&data_length.to_be_bytes()) {
        Ok(_) => {},
        Err(error) => return Err(error),
    }

    match stream.write_all(data) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error),
    }
}

pub fn receive_headered(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {

    let mut header_buffer: [u8; 8] = [0; 8];

    // read header bytes into buffer
    match stream.read_exact(&mut header_buffer) {
        Ok(_size) => println!("Read header"),
        Err(error) => {
            return Err(error) // this usually means the socket would have blocked
        }
    }

    // decode length of payload 
    let payload_length = u64::from_be_bytes(header_buffer);

    // allocate a vector with zeroes equal to size of payload
    let mut payload_buffer = vec![0u8; payload_length as usize];

    loop {
        // read socket buffer into vector passed as mutable slice
        match stream.read(payload_buffer.as_mut_slice()) {
            Ok(_) => {
                return Ok(payload_buffer)
            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => continue, // we dont want to block if we know we are supposed to be getting data
                    _ => return Err(error)
                }
            },
        }
    }
}