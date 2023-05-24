#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "testing")]
pub mod multi;

#[cfg(feature = "testing")]
pub use multi as multi_test;

pub const BLOCK_SIZE: usize = 256;

mod custom;
mod messages;

pub mod interfaces;
pub mod error;
pub use custom::*;
pub use messages::*;

#[cfg(feature = "snip20")]
pub mod snip20;
