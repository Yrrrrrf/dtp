// dtp/examples/main_example.rs

use bytes::Bytes;
use dtp::{Frame, Header, Packet, Segment};

fn main() {
    println!("--- DTP Simple Encapsulation Example ---");

    // 1. The Payload: Start with the data you want to send.
    let my_payload = Bytes::from_static(b"Hello, DTP!");
    println!("\n[1] Application Data: '{}'", String::from_utf8_lossy(&my_payload));

    // 2. Transport Layer: Encapsulate the data in a Segment.
    //    This layer adds source and destination port numbers.
    let transport_segment = Segment::new(
        Header::new(54321, 443), // Source Port (dynamic) -> Destination Port (HTTPS)
        my_payload,
    );
    println!("\n[2] Encapsulated into a Transport Segment...");

    // 3. Network Layer: Place the Segment into a Packet.
    //    This layer adds source and destination IP addresses.
    let network_packet = Packet::new(
        Header::new(0x7F000001, 0x08080808), // 127.0.0.1 -> 8.8.8.8 (Google DNS)
        vec![transport_segment],
    );
    println!("\n[3] Encapsulated into a Network Packet...");

    // 4. Data Link Layer: Place the Packet into a Frame.
    //    This is the final layer, adding MAC addresses for the local network link.
    let datalink_frame = Frame::new(
        Header::new(
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF], // Your device's MAC Address
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66], // The gateway/router's MAC Address
        ),
        vec![network_packet],
    );
    println!("\n[4] Encapsulated into a Data Link Frame...");

    // 5. Final Result: Print the fully constructed frame.
    //    The custom Display implementation shows the beautiful, nested structure.
    println!("\n--- Final Frame Ready for Transmission ---\n");
    println!("{}", datalink_frame);
}
