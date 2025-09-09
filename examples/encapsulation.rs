// dtp/examples/encapsulation.rs

use bytes::{Bytes, BytesMut};
use dtp::{Frame, Header, Packet, Segment};

// Define constants for the example
const SEGMENT_PAYLOAD_SIZE: usize = 32;
const SEGMENTS_PER_PACKET: usize = 2;

fn main() {
    println!("--- DTP Encapsulation Example ---");

    // 1. CREATE THE INITIAL DATA
    // This is the raw data your application wants to send.
    let original_data = Bytes::from(
        "This is the application payload that will be segmented, packetized, and framed for transport!",
    );
    println!("\n[1] Original Data ({} bytes):\n'{}'", original_data.len(), String::from_utf8_lossy(&original_data));

    // 2. BUILD THE TRANSPORT LAYER (SEGMENTS)
    // The data is chunked into smaller segments.
    let segments = create_segments(original_data.clone());
    println!("\n[2] Created {} Transport Layer Segments...", segments.len());

    // 3. BUILD THE NETWORK LAYER (PACKETS)
    // Segments are grouped into packets.
    let packets = create_packets(segments);
    println!("\n[3] Created {} Network Layer Packets...", packets.len());

    // 4. BUILD THE DATA LINK LAYER (FRAME)
    // All packets are placed into a single frame for transmission.
    let frame = create_frame(packets);
    println!("\n[4] Assembled Data Link Layer Frame.");

    // 5. DISPLAY THE FINAL STRUCTURE
    // The Display trait provides a readable, indented view of the entire stack.
    println!("\n--- Final Encapsulated Frame Structure ---\n{}", frame);

    // 6. DECAPSULATION
    // Extract the data to verify that the process is reversible.
    let extracted_data = extract_data(&frame);
    println!("\n[5] Extracted Data ({} bytes):\n'{}'", extracted_data.len(), String::from_utf8_lossy(&extracted_data));

    // 7. VERIFICATION
    assert_eq!(original_data, extracted_data);
    println!("\n--- Verification Success: Original and extracted data match! ---");
}

/// Chunks data into Segments with transport headers.
fn create_segments(data: Bytes) -> Vec<Segment> {
    data.chunks(SEGMENT_PAYLOAD_SIZE)
        .enumerate()
        .map(|(i, chunk)| Segment {
            header: Header::new(49152, 80), // Src Port (ephemeral), Dst Port (HTTP)
            payload: Bytes::copy_from_slice(chunk),
        })
        .collect()
}

/// Groups Segments into Packets with network headers.
fn create_packets(segments: Vec<Segment>) -> Vec<Packet> {
    segments
        .chunks(SEGMENTS_PER_PACKET)
        .map(|segment_chunk| Packet {
            header: Header::new(0xC0A80164, 0x08080808), // 192.168.1.100 -> 8.8.8.8
            pdu: segment_chunk.to_vec(),
        })
        .collect()
}

/// Wraps Packets in a final Frame with data link headers.
fn create_frame(packets: Vec<Packet>) -> Frame {
    Frame {
        header: Header::new(
            [0xAA, 0xBB, 0xCC, 0x11, 0x22, 0x33], // Source MAC
            [0xDD, 0xEE, 0xFF, 0x44, 0x55, 0x66], // Destination MAC
        ),
        network_pdu: packets,
    }
}

/// Extracts and reassembles the original data from a Frame.
fn extract_data(frame: &Frame) -> Bytes {
    let mut data = BytesMut::new();
    for packet in &frame.network_pdu {
        for segment in &packet.pdu {
            data.extend_from_slice(&segment.payload);
        }
    }
    data.freeze()
}
