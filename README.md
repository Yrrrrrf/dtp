<h1 align="center">
    <div>DTP (Digital Transfer Protocol)</div>
</h1>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/dtp.svg?logo=rust)](https://crates.io/crates/dtp)
[![Docs.rs](https://img.shields.io/badge/docs.rs-dtp-66c2a5)](https://docs.rs/dtp)
[![Crates.io Downloads](https://img.shields.io/crates/d/dtp)](https://crates.io/crates/dtp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub](https://img.shields.io/badge/github-Yrrrrrf%2Fdtp-58A6FF?logo=github)](https://github.com/Yrrrrrf/dtp)

</div>

**DTP** is a lightweight, foundational Rust crate for building simple, layered communication protocols. It provides a set of generic, reusable data structures that represent the different layers of a network stack, allowing developers to focus on the application logic rather than the boilerplate of data encapsulation.

The core of DTP is a set of structs—`Frame`, `Packet`, and `Segment`—that allow you to neatly package your data for transmission, inspired by the OSI model. It is designed to be transport-agnostic, meaning you can use it as the data-structuring backbone for any transmission medium, including serial interfaces or **custom network transports**.

> **This project is currently in an active prototyping and development phase.** The API is designed to be simple and extensible but is subject to change.

## Features

*   **Layered Architecture**: Clearly defined structs for the Data Link (`Frame`), Network (`Packet`), and Transport (`Segment`) layers.
*   **Generic by Design**: Easily define your own address types (e.g., `MacAddress`, `Ipv4Address`) using the provided macros.
*   **Efficient Payloads**: Utilizes `bytes::Bytes` for zero-copy payload handling.
*   **Extensible**: A simple, composable design that's easy to build upon for more complex protocols.

## Installation

Add DTP to your project's `Cargo.toml`:

```bash
cargo add dtp
```

## Usage

DTP provides the building blocks to structure your data. Here is a conceptual example of how you would encapsulate a piece of data for transmission.

```rust
use dtp::{Frame, Packet, Segment, Header};
use bytes::Bytes;

fn main() {
    // 1. Your application data (payload)
    let payload_data = Bytes::from("Hello, World!");

    // 2. Encapsulate data in a Transport Layer Segment with Port addresses
    let segment = Segment {
        header: Header::new(8080, 80), // Source Port, Destination Port
        payload: payload_data,
    };

    // 3. Encapsulate the Segment in a Network Layer Packet with IP addresses
    let packet = Packet {
        header: Header::new(0xC0A80001, 0x08080808), // 192.168.0.1 -> 8.8.8.8
        pdu: vec![segment],
    };

    // 4. Encapsulate the Packet in a Data Link Layer Frame with MAC addresses
    let frame = Frame {
        header: Header::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF], [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
        network_pdu: vec![packet],
    };

    println!("Constructed a DTP Frame!");
    println!("{}", frame);

    // Now, this 'frame' is ready to be serialized and sent over any medium
    // of your choice (e.g., audio, serial, TCP, etc.).
}
```

## License

This project is licensed under the MIT License - see the [**LICENSE**](LICENSE) file for details.