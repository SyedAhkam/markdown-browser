use std::io::{Read, Write};

use std::path::Path;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

use protocol::{Parcel, Enum};
use md_browser_protocol::*;

const SOCKET_ADRESS: &str = "127.0.0.1:3103";

fn handle_client(stream: TcpStream) {
    let protocol_settings = protocol::Settings::default();
    let mut protocol_connection = protocol::wire::stream::Connection::new(
        stream,
        protocol::wire::middleware::pipeline::default(),
        protocol_settings
    );
    
    protocol_connection.send_packet(&Packet::Handshake(Handshake)).unwrap();
    
    loop {
        if let Some(response) = protocol_connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }
   
   // loop {
    //     let mut buf = [0 as u8; 50];
    //
    //     match stream.read(&mut buf) {
    //         Ok(size) => {
    //             if size == 0 {
    //                 break;
    //             }
    //             println!("got {:?}", to_hex_string(Vec::from(&buf[0..size])));
    //             match Packet::from_raw_bytes(
    //                 &buf,
    //                 &protocol_settings
    //             ) {
    //                 Ok(packet) => {
    //                     println!("Received packet: {:?}", &packet);
    //                 },
    //                 Err(e) => eprintln!("Invalid packet received: {:?}", e)
    //             };
    //         },
    //         Err(_) => {
    //             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
    //             stream.shutdown(Shutdown::Both).unwrap();
    //         }
    //     };
    /* } */
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
