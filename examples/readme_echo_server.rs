use message_io::network::{Network, NetEvent, Transport};

fn main() {
    // Create a Network with an associated event queue for reading its events.
    let (mut network, mut events) = Network::split();

    // Listen for TCP, UDP and WebSocket messages.
    network.listen(Transport::FramedTcp, "0.0.0.0:3042").unwrap(); // Tcp encoded for packets
    network.listen(Transport::Udp, "0.0.0.0:3043").unwrap();
    network.listen(Transport::Ws, "0.0.0.0:3044").unwrap();

    loop {
        match events.receive() { // Read the next event or wait until have it.
            NetEvent::Message(endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(&data));
                network.send(endpoint, &data);
            },
            NetEvent::Connected(_endpoint) => println!("Client connected"), // Tcp or Ws
            NetEvent::Disconnected(_endpoint) => println!("Client disconnected"), //Tcp or Ws
        }
    }
}
