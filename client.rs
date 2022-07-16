use std::io::prelude::*;
use std::net::TcpStream;
use std::io;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read!");
        stream.write(input.as_bytes()).expect("Failed to write!");

        let mut _reader = BufReader::new(&stream);
        let mut _buffer: Vec<u8> = Vec::new();
        _reader.read_until(b'\n',&mut _buffer).expect("Failed to read into buffer");
        println!("read from server: {}", std::str::from_utf8(&_buffer).unwrap());
        println!("");
        //stream.write(&[1])?;
        //stream.read(&mut [0;128])?;
    }
    Ok(())
}
