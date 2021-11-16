use md_browser_protocol::{
    TcpConnection,
    ProtocolConnection,
    Packet,
    Hello,
    Request,
    Response,
    Goodbye,
    Url
};

fn wait_for_response(protocol_connection: &mut ProtocolConnection<Packet, std::net::TcpStream>) -> Response {
    loop {
        if let Some(data) = protocol_connection.receive_packet() {
            match data {
                Packet::Response(resp) => {
                    println!("received response: {:?}", resp);

                    return resp;
                },
                _ => ()
            };
        };

    }
}

fn request_md_doc(protocol_connection: &mut ProtocolConnection<Packet, std::net::TcpStream>, address: &str, filename: &str) {
    protocol_connection
        .send_packet(&Packet::Request(Request {
            url: Url::new(address, filename) 
        }))
}

fn make_protocol_connection(address: &str) -> Option<ProtocolConnection<Packet, std::net::TcpStream>> {
    match TcpConnection::new(address) {
        Some(tcp_connection) => Some(ProtocolConnection::new(tcp_connection.into_inner())),
        None => None
    }
}

pub fn start() {
    let address = "localhost:3103";
    if let Some(mut protocol_connection) = make_protocol_connection(&address) {
        request_md_doc(&mut protocol_connection, &address, "index.md");
        let response = wait_for_response(&mut protocol_connection);
    }

    println!("Starting browser..");
}
