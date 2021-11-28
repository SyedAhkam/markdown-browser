use std::path::PathBuf;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::fs;

use md_browser_protocol::*;

const SOCKET_ADDRESS: &str = "127.0.0.1:3103";

fn return_response(dir: PathBuf, protocol_connection: &mut ProtocolConnection<Packet, TcpStream>, request: &Request) {
    let filename = "index.md"; // TODO
    let markdown_document = Markdown::from(
        fs::read_to_string(dir.join(filename)).unwrap()
    );

    protocol_connection.send_packet(&Packet::Response(Response { md: markdown_document }));
}

fn handle_client(dir: PathBuf, stream: TcpStream) {
    println!("Client connected!");
    let mut protocol_connection = ProtocolConnection::new(stream);   
    protocol_connection.send_packet(&Packet::Handshake(Handshake));

    loop {
        if let Some(response) = protocol_connection.receive_packet() {
            println!("Recieved packet from client: {:?}", response);
            match response {
                Packet::Handshake(_) => (),
                Packet::Hello(_) => protocol_connection.send_packet(&Packet::Hello(Hello)),
                Packet::Request(req) => {
                    println!("request recieved for: {}", req.url);
                    return_response(dir.clone(), &mut protocol_connection, &req); // FIXME
                },
                Packet::Response(_) => (), // server sent the response itself
                Packet::Goodbye(_) => {
                    println!("bye");

                    protocol_connection
                        .into_inner()
                        .into_inner()
                        .shutdown(Shutdown::Both).unwrap(); // fixme
                    break;
                }
            }
        }
    }
}

pub fn run(dir: PathBuf) -> std::io::Result<()> {
    let listener = TcpListener::bind(SOCKET_ADDRESS)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let dir_owned = dir.to_owned();
                thread::spawn(move || {
                    handle_client(dir_owned, stream);
                });
            },
            Err(e) => eprintln!("Client failed to connect: {:?}", e)
        }
    }
    Ok(())
}
