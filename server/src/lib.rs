use std::io::{Read, Write};

use std::path::Path;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

const SOCKET_ADRESS: &str = "127.0.0.1:3103";

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = [0 as u8; 50];

        match stream.read(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                // TODO
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        };
    }
}

pub fn run(dir: &Path) -> std::io::Result<()> {
    let listener = TcpListener::bind(SOCKET_ADRESS)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            },
            Err(e) => eprintln!("Client failed to connect: {:?}", e)
        }
    }
    Ok(())
}
