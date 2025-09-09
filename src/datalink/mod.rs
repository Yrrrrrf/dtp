// dtp/src/datalink/mod.rs

// Declare the sub-modules within the datalink layer.
pub mod frame;

// Re-export important items so users can access them with `dtp::datalink::FrameKind`
// instead of the longer `dtp::datalink::frame::FrameKind`.
pub use frame::FrameKind;
