use md_browser_protocol::{
    TcpConnection,
    ProtocolConnection,
    Packet,
    Hello,
    Goodbye
};

fn make_protocol_connection(address: &str) {
    if let Some(tcp_connection) = TcpConnection::new(address) {
        let mut protocol_connection = ProtocolConnection::new(tcp_connection.into_inner());

        for _ in 1..=2 {
            std::thread::sleep(std::time::Duration::from_secs(5));
            protocol_connection.send_packet(&Packet::Hello(Hello));
        }

        protocol_connection.send_packet(&Packet::Goodbye(Goodbye { reason: "hehe".to_string() }))
    }
}

pub fn start() {
    make_protocol_connection("localhost:3103");
    println!("Starting browser..");
}
