// dtp/src/lib.rs

// The core of your library depends on the `bytes` crate for efficient payload handling
// and `dev_utils` for formatting in the Display implementation.
use bytes::Bytes;
use dev_utils::format::*;
use std::fmt::{self, Display, Formatter};


// Main layers (modules)
pub mod datalink;
pub mod network;
pub mod transport;


// --- GENERIC BUILDING BLOCKS ---

/// A generic container for a source and destination address pair.
pub type AddressPair<A> = (A, A);

/// A generic header containing a pair of addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header<A> {
    pub addresses: AddressPair<A>,
}

impl<A> Header<A> {
    /// Creates a new header with source and destination addresses.
    pub fn new(src: A, dst: A) -> Self {
        Self {
            addresses: (src, dst),
        }
    }
    /// Returns a reference to the source address.
    pub fn src(&self) -> &A {
        &self.addresses.0
    }
    /// Returns a reference to the destination address.
    pub fn dst(&self) -> &A {
        &self.addresses.1
    }
}

/// A macro to quickly define address types and their default `Header` implementations.
macro_rules! define_addresses {
    ($($(#[$meta_d:meta])* $name:ident: $inner:ty, $default:expr),* $(,)?) => {
        $(
            $(#[$meta_d])*
            pub type $name = $inner;

            impl Default for Header<$name> {
                fn default() -> Self {
                    let default_addr: $name = $default;
                    Self {addresses: (default_addr, default_addr),}
                }
            }
        )*
    };
}

// Define the concrete address types used in the protocol stack.
define_addresses! {
    /// Represents a Data Link Layer MAC address ([u8; 6]).
    MacAddress: [u8; 6], [0, 0, 0, 0, 0, 0],
    /// Represents a Network Layer IPv4 address (u32).
    Ipv4Address: u32, 0x7F000001, // 127.0.0.1
    /// Represents a Transport Layer Port address (u16).
    PortAddress: u16, 80,
}

/// A macro to implement the `IntoIterator` trait for the layered structs,
/// allowing easy iteration over their payloads.
macro_rules! impl_iterator_trait {
    ($name:ident, $payload_field:ident, $payload_ty:ty) => {
        // Implementation for consuming iteration: `for item in layer_struct { ... }`
        impl IntoIterator for $name {
            type Item = <$payload_ty as IntoIterator>::Item;
            type IntoIter = <$payload_ty as IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                self.$payload_field.into_iter()
            }
        }

        // Implementation for borrowing iteration: `for item in &layer_struct { ... }`
        impl<'a> IntoIterator for &'a $name {
            type Item = &'a <$payload_ty as IntoIterator>::Item;
            type IntoIter = std::slice::Iter<'a, <$payload_ty as IntoIterator>::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.$payload_field.iter()
            }
        }
    };
}

/// A macro to define a protocol layer struct (`Frame`, `Packet`, `Segment`).
/// This reduces boilerplate by creating the struct definition, `new` and `Default`
/// implementations, a rich `Display` implementation, and iterator support.
macro_rules! define_layer_struct {
    (
        $(
            $(#[$meta:meta])*
            $name:ident { header: $header_ty:ty, $payload_field:ident: $payload_ty:ty $(,)? }
        ),* $(,)?
    ) => {
        $(
            $(#[$meta])*
            #[derive(Clone, PartialEq, Debug)]
            pub struct $name {
                pub header: Header<$header_ty>,
                pub $payload_field: $payload_ty,
            }

            impl $name {
                /// Constructs a new instance of the layer.
                pub fn new(header: Header<$header_ty>, $payload_field: $payload_ty) -> Self {
                    Self {
                        header,
                        $payload_field,
                    }
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self {
                        header: Header::<$header_ty>::default(),
                        $payload_field: Default::default(),
                    }
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    writeln!( f, "{}", format!(
                        "src: {:X?} -> dst: {:X?} | {:>4} {} |",
                        self.header.addresses.0,
                        self.header.addresses.1,
                        self.$payload_field.len(),
                        // A bit of a trick to get a plural 's' correctly
                        if self.$payload_field.len() == 1 { stringify!($payload_field).trim_end_matches('s') } else { stringify!($payload_field) }
                    ).style(Style::Italic)
                    )?;
                    for (idx, item) in self.$payload_field.iter().enumerate() {
                        // Recursively print the nested layers with indentation
                        let item_str = format!("{}", item);
                        for line in item_str.lines() {
                             writeln!(f, "\t{}", line)?;
                        }
                    }
                    Ok(())
                }
            }

            // Implement iteration for the struct
            impl_iterator_trait!($name, $payload_field, $payload_ty);
        )*
    }
}


// --- PROTOCOL STACK DEFINITIONS ---

define_layer_struct! {
    /// Represents a Transport Layer Segment. It carries the raw application data payload.
    Segment { header: PortAddress, payload: Bytes },

    /// Represents a Network Layer Packet. It contains one or more `Segment`s.
    Packet { header: Ipv4Address, pdu: Vec<Segment> },

    /// Represents a Data Link Layer Frame. This is the final container, holding one or more `Packet`s.
    Frame { header: MacAddress, network_pdu: Vec<Packet> },
}


// --- CORE TRAITS ---

/// A trait for types that can be serialized into a byte vector.
pub trait ToBytes {
    /// Converts the structure to a byte representation.
    fn to_bytes(&self) -> Vec<u8>;
}

/// A trait for getting size information about a network layer.
pub trait LayerSize {
    /// Returns the size of the payload in bytes.
    fn payload_size(&self) -> usize;
    /// Returns the total size of the layer, including headers.
    fn total_size(&self) -> usize;
}

/// A generic builder trait.
pub trait LayerBuilder {
    type Output;
    /// Builds the final output from the builder's configuration.
    fn build(&self) -> Self::Output;
}
