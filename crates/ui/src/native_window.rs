use crate::placement::Origin;

pub struct Spot {
    pub at: (i32, i32),
    pub logical_size: (f32, f32),
    pub origin: Origin,
    pub activate: bool,
}

#[cfg(not(windows))]
mod other;
#[cfg(windows)]
mod win;

#[cfg(not(windows))]
pub use other::*;
#[cfg(windows)]
pub use win::*;
