// dtp/src/datalink/frame.rs

// Import the core structs from the crate's root.
use crate::{Frame, Packet, Segment};

/// Enum representing different framing strategies or types.
/// This allows the crate to eventually support various L2 protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    /// A simple bit-oriented frame with a start/end flag (e.g., HDLC-like).
    BitOriented { flag: u8 },
    /// A character-oriented frame using a SYNC byte.
    CharacterOriented { sync: u8 },
    /// A length-prefixed frame.
    LengthPrefixed,
}

impl Default for FrameKind {
    fn default() -> Self {
        // Default to a common flag value used in protocols like HDLC.
        // 0b01111110 == 0x7E
        FrameKind::BitOriented { flag: 0x7E }
    }
}

// Implementations specific to the `Frame` struct will live here.
impl Frame {
    // In the future, you can add methods like:
    // pub fn calculate_checksum(&self) -> u16 { ... }
    // pub fn from_bytes(bytes: &[u8], kind: FrameKind) -> Result<Self, Error> { ... }
}
