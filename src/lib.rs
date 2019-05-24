#![feature(marker_trait_attr)]

mod pure_clone;
mod mutbl;
mod vec;

pub use pure_clone::PureClone;
pub use mutbl::Mut;
pub use vec::MutVec;
