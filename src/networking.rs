use std::{io::{Error, Read, Write}, net::TcpStream};

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

pub fn send_headered(data: &[u8], stream: &mut TcpStream) -> Result<(), Error> {

    // compress data with lz4
    let compressed_data = compress_prepend_size(data);

    // make sure we measure the length of the compressed data, not the uncompressed data
    let data_length = compressed_data.len() as u64;

    match stream.write_all(&data_length.to_be_bytes()) {
        Ok(_) => {},
        Err(error) => return Err(error),
    }

    match stream.write_all(&compressed_data) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error),
    }
}

pub fn receive_headered(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {

    let mut header_buffer: [u8; 8] = [0; 8];

    // read header bytes into buffer
    match stream.read_exact(&mut header_buffer) {
        Ok(_size) => {},
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
        match stream.read_exact(payload_buffer.as_mut_slice()) {
            Ok(_bytes_read) => {

                // decompress payload
                let decompressed_payload = decompress_size_prepended(&payload_buffer).expect("Failed to decompress update");

                return Ok(decompressed_payload)
            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        continue;
                    }, // we dont want to block if we know we are supposed to be getting data
                    _ => return Err(error)
                }
            },
        }
    }
}